use std::sync::Arc;

use async_trait::async_trait;
use failure::Error;

use crate::library::MetaValue;

pub type SharedStorageBackend = Arc<Box<dyn StorageBackend>>;

#[async_trait]
pub trait StorageBackend: Sync + Send + ::std::fmt::Debug {
    async fn open_collection(&self, name: &str) -> Result<Box<dyn StorageCollection>, Error>;
    // TODO: this doesn't work because StorageBackend is used as a Trait Object
    // I need to think about an api that works so the memory library can store a snapshot in the storage folder
    // async fn write_collection(&self, name: &str, value: &impl Serialize) -> Result<(), Error> {
    //     unimplemented!()
    // }
    // async fn read_collection<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>, Error>;
}

#[async_trait]
pub trait StorageCollection: Sync + Send + ::std::fmt::Debug {
    async fn read(&self, name: &str) -> Result<Option<MetaValue>, Error>;
    async fn write(&self, name: &str, value: MetaValue) -> Result<(), Error>;
}
