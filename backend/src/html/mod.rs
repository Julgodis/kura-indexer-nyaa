use crate::{data, rss};

pub mod view;
pub mod list;

pub fn parse_list(data: &str) -> anyhow::Result<Vec<data::Item>> {
    Ok(list::parse(data)?)
}

pub fn parse_view(data: &str) -> anyhow::Result<data::View> {
    Ok(view::parse(data)?)
}

pub fn parse_human_size(size: &str) -> anyhow::Result<u64> {
    let size = size.trim();
    if let Some(size) = size.strip_suffix(" MiB") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok((size * 1024.0 * 1024.0) as u64);
    } else if let Some(size) = size.strip_suffix(" GiB") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok((size * 1024.0 * 1024.0 * 1024.0) as u64);
    } else if let Some(size) = size.strip_suffix(" KiB") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok((size * 1024.0) as u64);
    } else if let Some(size) = size.strip_suffix(" B") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok(size as u64);
    } else if let Some(size) = size.strip_suffix("1 Byte") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok(size as u64);
    } else if let Some(size) = size.strip_suffix(" TiB") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok((size * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64);
    } else if let Some(size) = size.strip_suffix(" PiB") {
        let size = size.trim();
        let size = size
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("invalid size: {}", size))?;
        return Ok((size * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64);    
    } else {
        return Err(anyhow::anyhow!("invalid size: {}", size))?;
    }
}
