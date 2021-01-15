use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;
use failure::Error;
use serde_json;
use tokio::fs;

use rustic_core::library::MetaValue;
use rustic_core::{StorageBackend, StorageCollection};

// TODO: track open files
#[derive(Debug, Clone)]
pub struct JsonStorage {
    folder_path: String,
}

impl JsonStorage {
    pub fn new(folder_path: String) -> Result<Self, failure::Error> {
        let path = Path::new(&folder_path);
        if !path.exists() {
            ::std::fs::create_dir(path)?;
        }
        Ok(JsonStorage { folder_path })
    }

    fn get_collection_path(&self, name: &str) -> String {
        format!("{}/{}.json", &self.folder_path, name)
    }
}

#[async_trait]
impl StorageBackend for JsonStorage {
    async fn open_collection(&self, name: &str) -> Result<Box<dyn StorageCollection>, Error> {
        let path = self.get_collection_path(name);
        let collection = JsonFile::new(path);

        Ok(Box::new(collection))
    }

    // TODO: this doesn't work because StorageBackend is used as a Trait Object
    // I need to think about an api that works so the memory library can store a snapshot in the storage folder
    // async fn write_collection<T: Serialize>(&self, name: &str, value: &T) -> Result<(), Error> {
    //     let file_path = self.get_collection_path(name);
    //     fs::write(file_path, serde_json::to_string(value)?).await?;
    //     Ok(())
    // }

    // async fn read_collection<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>, Error> {
    //     let file_path = self.get_collection_path(name);
    //     if !Path::new(&file_path).exists() {
    //         return Ok(None);
    //     }
    //     let content = fs::read_to_string(file_path)?;
    //     let collection = serde_json::from_str(&content)?;
    //
    //     Ok(Some(collection))
    // }
}

#[derive(Debug, Clone)]
struct JsonFile {
    path: String,
}

impl JsonFile {
    fn new(path: String) -> Self {
        JsonFile { path }
    }

    async fn read_file(&self) -> Result<Option<HashMap<String, MetaValue>>, Error> {
        if !Path::new(&self.path).exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&self.path).await?;
        let collection = serde_json::from_str(&content)?;
        Ok(Some(collection))
    }
}

#[async_trait]
impl StorageCollection for JsonFile {
    async fn read(&self, name: &str) -> Result<Option<MetaValue>, Error> {
        let collection = self.read_file().await?;
        let value = collection.and_then(|collection| collection.get(name).cloned());

        Ok(value)
    }

    async fn write(&self, name: &str, value: MetaValue) -> Result<(), Error> {
        let mut collection = self.read_file().await?.unwrap_or_default();
        collection.insert(name.to_string(), value);

        fs::write(&self.path, serde_json::to_string(&collection)?).await?;
        Ok(())
    }
}
