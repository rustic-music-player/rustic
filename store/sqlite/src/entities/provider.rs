use failure::Error;
use rustic_core::ProviderType;

pub struct SerializedProvider(i32);

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

pub fn int_to_provider(provider: i32) -> Result<ProviderType, Error> {
    match provider {
        0 => Ok(ProviderType::Pocketcasts),
        1 => Ok(ProviderType::Soundcloud),
        2 => Ok(ProviderType::GooglePlayMusic),
        3 => Ok(ProviderType::Spotify),
        4 => Ok(ProviderType::LocalMedia),
        5 => Ok(ProviderType::Youtube),
        _ => Err(format_err!("Unknown provider type {}", provider)),
    }
}
