use failure::{Error, format_err};
use keyring::{Keyring, KeyringError};

use async_trait::async_trait;
use rustic_core::{Credentials, CredentialStore, ProviderType};

pub struct KeychainCredentialStore;

impl KeychainCredentialStore {
    fn handle_error(err: KeyringError) -> Error {
        match err {
            KeyringError::NoPasswordFound => format_err!("No password found"),
            #[cfg(target_os = "macos")]
            KeyringError::MacOsKeychainError(e) => format_err!("Keyring access failed {:?}", e),
            #[cfg(target_os = "linux")]
            KeyringError::SecretServiceError(e) => format_err!("Keyring access failed {:?}", e),
            #[cfg(target_os = "windows")]
            KeyringError::WindowsVaultError => format_err!("Keyring access failed"),
            KeyringError::NoBackendFound => format_err!("Keyring access failed"),
            KeyringError::Parse(e) => e.into(),
        }
    }
}

#[async_trait]
impl CredentialStore for KeychainCredentialStore {
    async fn get_credentials(&self, provider: ProviderType) -> Result<Option<Credentials>, Error> {
        let provider = format!("{:?}", provider);
        let keyring = Keyring::new("rustic", &provider);
        match keyring.get_password() {
            Ok(password) => {
                let credentials = serde_json::from_str(&password)?;

                Ok(credentials)
            },
            Err(KeyringError::NoPasswordFound) => Ok(None),
            Err(e) => Err(KeychainCredentialStore::handle_error(e))
        }
    }

    async fn store_credentials(&self, provider: ProviderType, credentials: Credentials) -> Result<(), Error> {
        let provider = format!("{:?}", provider);
        let keyring = Keyring::new("rustic", &provider);
        let password = serde_json::to_string(&credentials)?;
        match keyring.set_password(&password) {
            Ok(_) => Ok(()),
            Err(e) => Err(KeychainCredentialStore::handle_error(e))
        }
    }
}
