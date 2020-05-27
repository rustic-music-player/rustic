use std::collections::HashMap;
use std::path::{Path, PathBuf};

use failure::Error;
use log::error;
use pinboard::NonEmptyPinboard;
use tokio::fs;

use async_trait::async_trait;
use rustic_core::{Credentials, CredentialStore, ProviderType};

pub struct FileCredentialStore {
    credentials: NonEmptyPinboard<HashMap<ProviderType, Credentials>>,
    path: PathBuf,
}

impl FileCredentialStore {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let credentials = match fs::read_to_string(&path).await {
            Ok(store) => serde_json::from_str(&store)?,
            Err(e) => {
                error!("Error reading credentials store {:?}", e);
                HashMap::new()
            }
        };
        let path = path.as_ref().to_path_buf();
        let credentials = NonEmptyPinboard::new(credentials);

        Ok(FileCredentialStore {
            credentials,
            path,
        })
    }

    pub async fn save(&self) -> Result<(), Error> {
        let credentials = self.credentials.read();
        let store = serde_json::to_string(&credentials)?;
        fs::write(&self.path, store).await?;

        Ok(())
    }
}

#[async_trait]
impl CredentialStore for FileCredentialStore {
    async fn get_credentials(&self, provider: ProviderType) -> Result<Option<Credentials>, Error> {
        let credentials = self.credentials.read();
        Ok(credentials.get(&provider).cloned())
    }

    async fn store_credentials(&self, provider: ProviderType, credentials: Credentials) -> Result<(), Error> {
        let mut credentials_map = self.credentials.read();
        credentials_map.insert(provider, credentials);
        self.credentials.set(credentials_map);

        self.save().await?;

        Ok(())
    }
}
