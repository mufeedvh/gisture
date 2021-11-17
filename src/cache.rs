use std::path::Path;

use cacache;

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub permalink_key: String,
    pub updated_at: String,
}

// NOTE: cache function calls are blocking (forced sync)
impl Cache {
    /// Save a blog entry to disk cache
    pub async fn save_cache_entry(&self) -> Result<(), cacache::Error> {
        let cache_dir = String::from("./gisture_cache");

        cacache::write(&cache_dir, &self.permalink_key, self.updated_at.as_bytes()).await?;

        Ok(())
    }

    /// Check if a blog entry is cached on disk
    pub async fn is_cached(&self) -> Result<bool, cacache::Error> {
        let cache_dir = String::from("./gisture_cache");
        let data = cacache::read(&cache_dir, &self.permalink_key).await?;

        let build_file = Path::new(&format!("public/{}", &self.permalink_key)).exists();

        if data == self.updated_at.as_bytes() && build_file {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
