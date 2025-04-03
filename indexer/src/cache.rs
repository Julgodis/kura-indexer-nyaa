use reqwest::Url;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct CacheEntryMetadata {
    uuid: Uuid,
    expiration: chrono::DateTime<chrono::Utc>,
    data_size: u64,
}

type CacheKey = (String, String);

#[derive(Debug)]
pub struct Cache {
    base_dir: PathBuf,
    max_size: u64,
    total_size: u64,
    metadata: HashMap<CacheKey, CacheEntryMetadata>,
}


impl Cache {
    pub fn new(base_dir: PathBuf, max_size: u64) -> io::Result<Self> {
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        } else if !base_dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a directory", base_dir.display()),
            ));
        }

        for entry in fs::read_dir(&base_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                match fs::remove_file(entry.path()) {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("failed to remove file {}: {}", entry.path().display(), e);
                    }
                }
            }
        }

        Ok(Self {
            base_dir,
            max_size,
            total_size: 0,
            metadata: HashMap::new(),
        })
    }

    fn file_path(&self, uuid: Uuid) -> PathBuf {
        self.base_dir.join(uuid.to_string())
    }

    fn remove_entry(&mut self, url_key: &CacheKey) {
        if let Some(meta) = self.metadata.remove(url_key) {
            let path = self.file_path(meta.uuid);
            let _ = fs::remove_file(path);
            self.total_size = self.total_size.saturating_sub(meta.data_size);
        }
    }

    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self
            .metadata
            .iter()
            .min_by_key(|(_, meta)| meta.expiration)
            .map(|(k, meta)| (k.clone(), meta.clone()))
        {
            self.remove_entry(&oldest_key);
        }
    }

    pub fn cleanup(&mut self) {
        let now = chrono::Utc::now();
        let expired_keys: Vec<CacheKey> = self
            .metadata
            .iter()
            .filter(|(_, meta)| now > meta.expiration)
            .map(|(key, _)| key.clone())
            .collect();
        for key in expired_keys {
            self.remove_entry(&key);
        }
    }

    pub fn put_inner<T, Q>(&mut self, url: &Url, query: &Q, lifetime: Duration, data: &T) -> io::Result<()>
    where
        T: Serialize,
        Q: Serialize,
    {
        let key = (url.to_string(), serde_json::to_string(query).ok().unwrap_or_default());

        let data_bytes =
            serde_json::to_vec(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let data_size = data_bytes.len() as u64;

        self.cleanup();

        while self.total_size + data_size > self.max_size && !self.metadata.is_empty() {
            self.evict_oldest();
        }

        if self.total_size + data_size > self.max_size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Not enough space in cache",
            ));
        }

        let uuid = Uuid::new_v4();
        let file_path = self.file_path(uuid);
        fs::write(&file_path, data_bytes)?;

        let expiration = chrono::Utc::now() + chrono::Duration::from_std(lifetime).unwrap();
        tracing::trace!(
            "wrote cache entry to {} (valid until {})",
            file_path.display(),
            expiration.to_rfc3339()
        );

        self.metadata.insert(
            key,
            CacheEntryMetadata {
                uuid,
                expiration,
                data_size,
            },
        );
        self.total_size += data_size;

        Ok(())
    }

    pub fn put<T, Q>(&mut self, url: &Url, query: &Q, lifetime: Duration, data: &T)
    where
        T: Serialize,
        Q: Serialize,
    {
        if let Err(e) = self.put_inner(url, query, lifetime, data) {
            tracing::warn!("failed to put cache entry: {}", e);
        }
    }

    pub fn get<T, Q>(&mut self, url: &Url, query: &Q) -> Option<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let key = (url.to_string(), serde_json::to_string(query).ok().unwrap_or_default());

        self.cleanup();

        if let Some(meta) = self.metadata.get(&key) {
            let now = chrono::Utc::now();
            if now > meta.expiration {
                tracing::trace!("cache entry expired: {:?}", key);
                self.remove_entry(&key);
                return None;
            }
            tracing::trace!(
                "cache entry found: {:?} (valid util {}, {} seconds remaining)",
                key,
                meta.expiration.to_rfc3339(),
                (meta.expiration - now).num_seconds()
            );
            let file_path = self.file_path(meta.uuid);
            let data_bytes = fs::read(file_path).ok()?;
            let data = serde_json::from_slice(&data_bytes).ok()?;
            return Some(data);
        }
        None
    }
}
