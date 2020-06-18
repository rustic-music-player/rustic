use std::io::Write;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;

use rustic_core::ProviderType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SerializedProvider(ProviderType);

impl From<ProviderType> for SerializedProvider {
    fn from(p: ProviderType) -> Self {
        SerializedProvider(p)
    }
}

impl From<SerializedProvider> for ProviderType {
    fn from(p: SerializedProvider) -> Self {
        p.0
    }
}

impl<DB> FromSql<Integer, DB> for SerializedProvider
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        let provider = int_to_provider(i32::from_sql(bytes)?);
        Ok(SerializedProvider(provider))
    }
}

impl<DB> ToSql<Integer, DB> for SerializedProvider
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let p = provider_to_int(self.0);
        p.to_sql(out)
    }
}

pub fn provider_to_int(provider: ProviderType) -> i32 {
    match provider {
        ProviderType::Internal => 0,
        ProviderType::Pocketcasts => 1,
        ProviderType::Soundcloud => 2,
        ProviderType::GooglePlayMusic => 3,
        ProviderType::Spotify => 4,
        ProviderType::LocalMedia => 5,
        ProviderType::Youtube => 6,
    }
}

pub fn int_to_provider(provider: i32) -> ProviderType {
    match provider {
        0 => ProviderType::Internal,
        1 => ProviderType::Pocketcasts,
        2 => ProviderType::Soundcloud,
        3 => ProviderType::GooglePlayMusic,
        4 => ProviderType::Spotify,
        5 => ProviderType::LocalMedia,
        6 => ProviderType::Youtube,
        _ => unreachable!("someone tampered with the data"),
    }
}
