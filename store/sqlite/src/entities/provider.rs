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
        match i32::from_sql(bytes)? {
            0 => Ok(ProviderType::Pocketcasts),
            1 => Ok(ProviderType::Soundcloud),
            2 => Ok(ProviderType::GooglePlayMusic),
            3 => Ok(ProviderType::Spotify),
            4 => Ok(ProviderType::LocalMedia),
            5 => Ok(ProviderType::Youtube),
            _ => Err(format!("someone tampered with the data").into()),
        }
        .map(SerializedProvider::from)
    }
}

impl<DB> ToSql<Integer, DB> for SerializedProvider
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let p = match self.0 {
            ProviderType::Pocketcasts => 0,
            ProviderType::Soundcloud => 1,
            ProviderType::GooglePlayMusic => 2,
            ProviderType::Spotify => 3,
            ProviderType::LocalMedia => 4,
            ProviderType::Youtube => 5,
        };
        p.to_sql(out)
    }
}

pub fn provider_to_int(provider: ProviderType) -> i32 {
    match provider {
        ProviderType::Pocketcasts => 0,
        ProviderType::Soundcloud => 1,
        ProviderType::GooglePlayMusic => 2,
        ProviderType::Spotify => 3,
        ProviderType::LocalMedia => 4,
        ProviderType::Youtube => 5,
    }
}

pub fn int_to_provider(provider: i32) -> ProviderType {
    match provider {
        0 => ProviderType::Pocketcasts,
        1 => ProviderType::Soundcloud,
        2 => ProviderType::GooglePlayMusic,
        3 => ProviderType::Spotify,
        4 => ProviderType::LocalMedia,
        5 => ProviderType::Youtube,
        _ => unreachable!("someone tampered with the data"),
    }
}
