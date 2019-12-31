use failure::Error;
use gmusic::GoogleMusicApi;
use serde_derive::Deserialize;

use rustic_core::{provider, SharedLibrary, Track};

mod playlist;

#[derive(Clone, Deserialize, Debug)]
pub struct GooglePlayMusicProvider {
    client_id: String,
    client_secret: String,
    device_id: String,
    #[serde(skip)]
    client: Option<GoogleMusicApi>
}

impl provider::ProviderInstance for GooglePlayMusicProvider {
    fn setup(&mut self) -> Result<(), Error> {
        let mut api = GoogleMusicApi::new(self.client_id.clone(), self.client_secret.clone());
        api.load_token()
            .or_else(|_| api.login().and_then(|_| api.store_token()))?;
        self.client = Some(api);

        Ok(())
    }

    fn title(&self) -> &'static str {
        "Google Play Music"
    }

    fn uri_scheme(&self) -> &'static str {
        "gmusic"
    }

    fn provider(&self) -> provider::Provider {
        provider::Provider::GooglePlayMusic
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client.as_ref().unwrap();
        let playlists = client.get_all_playlists()?;

        Ok(provider::SyncResult {
            tracks: 0,
            artists: 0,
            albums: 0,
            playlists: playlists.len()
        })
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }

    fn navigate(&self, _path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        Ok(self.root())
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        unimplemented!()
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        unimplemented!()
    }

    fn stream_url(&self, _track: &Track) -> Result<String, Error> {
        unimplemented!()
    }
}
