use maplit::hashmap;

use rustic_core::{provider::ProviderItem, Artist, ProviderType};

use crate::meta::META_SOUNDCLOUD_USER_ID;

#[derive(Debug, Clone)]
pub struct SoundcloudUser(soundcloud::User);

impl From<soundcloud::User> for SoundcloudUser {
    fn from(user: soundcloud::User) -> Self {
        SoundcloudUser(user)
    }
}

impl From<SoundcloudUser> for Artist {
    fn from(user: SoundcloudUser) -> Self {
        let user = user.0;
        Artist {
            id: None,
            name: user.username,
            image_url: Some(user.avatar_url.replace("large", "t500x500")),
            uri: format!("soundcloud://user/{}", user.id),
            meta: hashmap!(
                META_SOUNDCLOUD_USER_ID.into() => user.id.into()
            ),
            provider: ProviderType::Soundcloud,
            albums: Vec::new(),
            playlists: Vec::new(),
            description: user.description,
        }
    }
}

impl From<SoundcloudUser> for ProviderItem {
    fn from(user: SoundcloudUser) -> Self {
        let artist = Artist::from(user);
        artist.into()
    }
}
