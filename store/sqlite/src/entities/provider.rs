use failure::Error;
use rustic_core::Provider;

pub struct SerializedProvider(i32);

pub fn provider_to_int(provider: Provider) -> i32 {
    match provider {
        Provider::Pocketcasts => 0,
        Provider::Soundcloud => 1,
        Provider::GooglePlayMusic => 2,
        Provider::Spotify => 3,
        Provider::LocalMedia => 4,
    }
}

pub fn int_to_provider(provider: i32) -> Result<Provider, Error> {
    match provider {
        0 => Ok(Provider::Pocketcasts),
        1 => Ok(Provider::Soundcloud),
        2 => Ok(Provider::GooglePlayMusic),
        3 => Ok(Provider::Spotify),
        4 => Ok(Provider::LocalMedia),
        _ => Err(format_err!("Unknown provider type {}", provider)),
    }
}
