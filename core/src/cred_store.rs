use async_trait::async_trait;
use failure::{Error, format_err};
use crate::ProviderType;
use std::fmt;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Clone, Deserialize, Serialize)]
pub enum Credentials {
    UserPass {
        username: String,
        password: String
    },
    Token(String)
}

impl Credentials {
    pub fn password(username: String, password: String) -> Self {
        Credentials::UserPass {
            username,
            password
        }
    }

    pub fn token<T: Serialize>(token: T) -> Result<Self, Error> {
        let token = serde_json::to_string(&token)?;

        Ok(Credentials::Token(token))
    }

    pub fn get_token<T: DeserializeOwned>(self) -> Result<T, Error> {
        match self {
            Credentials::Token(token) => {
                let token = serde_json::from_str(&token)?;

                Ok(token)
            },
            _ => Err(format_err!("Credentials are not a token"))
        }
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Credentials::UserPass { username, password: _ } => {
                f.debug_struct("Credentials::UserPass")
                    .field("username", username)
                    .finish()
            },
            Credentials::Token(_) => f.debug_struct("Credentials::Token").finish()
        }
    }
}

#[async_trait]
pub trait CredentialStore: Send + Sync {
    async fn get_credentials(&self, provider: ProviderType) -> Result<Option<Credentials>, Error>;

    async fn store_credentials(&self, provider: ProviderType, credentials: Credentials) -> Result<(), Error>;
}
