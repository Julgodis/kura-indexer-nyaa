use reqwest::Url;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// In-memory metadata for a cache entry.
#[derive(Debug, Clone)]
struct CacheEntryMetadata {
    /// Unique identifier for the data file.
    uuid: Uuid,
    /// Timestamp (seconds since UNIX_EPOCH) when this entry was created.
    created: u64,
    /// Lifetime (expiration duration) in seconds.
    lifetime: u64,
    /// Size in bytes of the stored data.
    data_size: u64,
}

/// A filesystem cache that stores arbitrary data on disk while keeping metadata in memory.
///
/// On creation, the cache directory is cleared. Each new entry is stored as a file named with a UUID.
/// The cache maintains a total size counter and evicts the oldest entries when the max_size is exceeded.
#[derive(Debug)]
pub struct Cache {
    base_dir: PathBuf,
    max_size: u64,
    total_size: u64,
    // Mapping from URL (as a string) to its metadata.
    metadata: HashMap<String, CacheEntryMetadata>,
}

impl Cache {
    /// Creates a new cache with the given base directory and maximum total size.
    /// On start, the base directory is cleared.
    pub fn new(base_dir: PathBuf, max_size: u64) -> io::Result<Self> {
        // Remove all files in the base directory.
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

    /// Returns the current time as seconds since the UNIX epoch.
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    /// Returns the full path for a file identified by the given UUID.
    fn file_path(&self, uuid: Uuid) -> PathBuf {
        self.base_dir.join(uuid.to_string())
    }

    /// Removes a cache entry (both in-memory metadata and its data file) given its URL key.
    fn remove_entry(&mut self, url_key: &str) {
        if let Some(meta) = self.metadata.remove(url_key) {
            let path = self.file_path(meta.uuid);
            let _ = fs::remove_file(path);
            self.total_size = self.total_size.saturating_sub(meta.data_size);
        }
    }

    /// Evicts the oldest cache entry (by creation time) if any exist.
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self
            .metadata
            .iter()
            .min_by_key(|(_, meta)| meta.created)
            .map(|(k, meta)| (k.clone(), meta.clone()))
        {
            self.remove_entry(&oldest_key);
        }
    }

    /// Cleans up expired entries.
    /// Iterates over all in-memory metadata, removing those entries whose lifetime has passed.
    pub fn cleanup(&mut self) {
        let now = Self::now_secs();
        let expired_keys: Vec<String> = self
            .metadata
            .iter()
            .filter(|(_, meta)| now > meta.created + meta.lifetime)
            .map(|(key, _)| key.clone())
            .collect();
        for key in expired_keys {
            self.remove_entry(&key);
        }
    }

    /// Inserts a new cache entry for the given URL.
    ///
    /// - `lifetime` specifies how long the entry is valid.
    /// - `data` is the content to be cached (serialized as JSON).
    ///
    /// Before writing, expired entries are removed and oldest entries are evicted until there is
    /// enough space to store the new data.
    pub fn put<T>(&mut self, url: &Url, lifetime: Duration, data: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        // Serialize data to JSON.
        let data_bytes =
            serde_json::to_vec(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let data_size = data_bytes.len() as u64;

        // Remove expired entries.
        self.cleanup();

        // Evict oldest entries until there is room for the new data.
        while self.total_size + data_size > self.max_size && !self.metadata.is_empty() {
            self.evict_oldest();
        }

        // If there's still not enough room, return an error.
        if self.total_size + data_size > self.max_size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Not enough space in cache",
            ));
        }

        // Generate a new UUID for this entry.
        let uuid = Uuid::new_v4();
        let now = Self::now_secs();

        // Write data to disk.
        let file_path = self.file_path(uuid);
        fs::write(&file_path, data_bytes)?;

        // Insert metadata for the new entry.
        self.metadata.insert(
            url.to_string(),
            CacheEntryMetadata {
                uuid,
                created: now,
                lifetime: lifetime.as_secs(),
                data_size,
            },
        );
        self.total_size += data_size;

        Ok(())
    }

    /// Retrieves a cached entry for the given URL, if it exists and has not expired.
    ///
    /// If the entry is expired, it is removed and `None` is returned.
    pub fn get<T>(&mut self, url: &Url) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let key = url.to_string();

        // First, clean up any expired entries.
        self.cleanup();

        if let Some(meta) = self.metadata.get(&key) {
            let now = Self::now_secs();
            if now > meta.created + meta.lifetime {
                // Entry expired; remove it.
                self.remove_entry(&key);
                return None;
            }
            let file_path = self.file_path(meta.uuid);
            let data_bytes = fs::read(file_path).ok()?;
            let data = serde_json::from_slice(&data_bytes).ok()?;
            return Some(data);
        }
        None
    }
}
