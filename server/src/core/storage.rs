use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

use crate::core::config::Config;
use crate::Result;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Upload file to storage backend
    async fn upload(&self, path: &str, data: &[u8]) -> Result<()>;

    /// Download file from storage backend
    async fn download(&self, path: &str) -> Result<Vec<u8>>;

    /// Delete file from storage backend
    async fn delete(&self, path: &str) -> Result<()>;

    /// Check if file exists
    async fn exists(&self, path: &str) -> Result<bool>;

    /// Get file size
    async fn size(&self, path: &str) -> Result<u64>;
}

/// Local filesystem storage (for development)
pub struct LocalStorage {
    base_path: String,
}

impl LocalStorage {
    pub fn new(base_path: String) -> Self {
        LocalStorage { base_path }
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn upload(&self, path: &str, data: &[u8]) -> Result<()> {
        let full_path = format!("{}/{}", self.base_path, path);
        let dir = Path::new(&full_path).parent().unwrap();
        fs::create_dir_all(dir).await?;
        fs::write(&full_path, data).await?;
        Ok(())
    }

    async fn download(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = format!("{}/{}", self.base_path, path);
        fs::read(&full_path)
            .await
            .map_err(|e| crate::Error::Storage(format!("Failed to read file: {}", e)))
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full_path = format!("{}/{}", self.base_path, path);
        fs::remove_file(&full_path)
            .await
            .map_err(|e| crate::Error::Storage(format!("Failed to delete file: {}", e)))
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        let full_path = format!("{}/{}", self.base_path, path);
        Ok(fs::try_exists(&full_path).await.unwrap_or(false))
    }

    async fn size(&self, path: &str) -> Result<u64> {
        let full_path = format!("{}/{}", self.base_path, path);
        let metadata = fs::metadata(&full_path).await?;
        Ok(metadata.len())
    }
}

/// NAS storage via NFS mount
pub struct NasStorage {
    mount_path: String,
}

impl NasStorage {
    pub fn new(mount_path: String) -> Self {
        NasStorage { mount_path }
    }
}

#[async_trait]
impl StorageBackend for NasStorage {
    async fn upload(&self, path: &str, data: &[u8]) -> Result<()> {
        let full_path = format!("{}/{}", self.mount_path, path);
        let dir = Path::new(&full_path).parent().unwrap();
        fs::create_dir_all(dir).await?;
        fs::write(&full_path, data).await?;
        Ok(())
    }

    async fn download(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = format!("{}/{}", self.mount_path, path);
        fs::read(&full_path)
            .await
            .map_err(|e| crate::Error::Storage(format!("Failed to read from NAS: {}", e)))
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full_path = format!("{}/{}", self.mount_path, path);
        fs::remove_file(&full_path)
            .await
            .map_err(|e| crate::Error::Storage(format!("Failed to delete from NAS: {}", e)))
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        let full_path = format!("{}/{}", self.mount_path, path);
        Ok(fs::try_exists(&full_path).await.unwrap_or(false))
    }

    async fn size(&self, path: &str) -> Result<u64> {
        let full_path = format!("{}/{}", self.mount_path, path);
        let metadata = fs::metadata(&full_path).await?;
        Ok(metadata.len())
    }
}

/// Create appropriate storage backend based on config
pub async fn create_backend(config: &Config) -> Box<dyn StorageBackend> {
    match config.storage_backend.as_str() {
        "nfs" | "nas" => {
            let path = config.nas_mount_path.clone().unwrap_or_else(|| {
                tracing::warn!("NAS_MOUNT_PATH not set, falling back to local storage");
                config.storage_path.clone()
            });
            Box::new(NasStorage::new(path))
        }
        "local" | _ => Box::new(LocalStorage::new(config.storage_path.clone())),
    }
}

impl std::fmt::Debug for dyn StorageBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageBackend").finish()
    }
}
