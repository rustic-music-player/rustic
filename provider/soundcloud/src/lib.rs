#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rustic_core as rustic;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate soundcloud;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

use failure::Error;

use rustic::library::{Album, MetaValue, Playlist, SharedLibrary, Track};
use rustic::provider;
use track::SoundcloudTrack;

mod error;
mod meta;
mod playlist;
mod track;

lazy_static! {
    static ref SOUNDCLOUD_RESOLVE_REGEX: regex::Regex = regex::RegexBuilder::new("^/([a-z]+)/([0-9]+)")
        .case_insensitive(true)
        .build()
        .unwrap();
}

const TRACK_URI_PREFIX: &str = "soundcloud://track/";

#[derive(Debug, Clone, Deserialize)]
pub struct SoundcloudProvider {
    client_id: String,
    auth_token: Option<String>,
}

impl SoundcloudProvider {
    fn client(&self) -> soundcloud::Client {
        let mut client = soundcloud::Client::new(self.client_id.as_str());
        if let Some(token) = self.auth_token.as_ref() {
            client.authenticate_with_token(token.clone());
        }
        client
    }
}

impl provider::ProviderInstance for SoundcloudProvider {
    fn setup(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn title(&self) -> &'static str {
        "Soundcloud"
    }

    fn uri_scheme(&self) -> &'static str {
        "soundcloud"
    }

    fn provider(&self) -> provider::Provider {
        provider::Provider::Soundcloud
    }

    fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client();
        let mut playlists: Vec<Playlist> = client
            .playlists()?
            .iter()
            .cloned()
            .map(playlist::SoundcloudPlaylist::from)
            .map(Playlist::from)
            .collect();
        library.sync_playlists(&mut playlists)?;
        Ok(provider::SyncResult {
            tracks: 0,
            albums: 0,
            artists: 0,
            playlists: playlists.len(),
        })
    }
    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec!["Likes".to_owned()],
            items: vec![],
        }
    }
    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        match path[0].as_str() {
            "Likes" => {
                let client = self.client();
                let likes = client.likes()?;
                let items = likes
                    .iter()
                    .cloned()
                    .filter(|like| like.track.is_some() || like.playlist.is_some())
                    .map(|like| (like.track, like.playlist))
                    .map(|like| match like {
                        (Some(track), _) => {
                            provider::ProviderItem::from(Track::from(SoundcloudTrack::from(track)))
                        }
                        (_, Some(playlist)) => provider::ProviderItem::from(Playlist::from(
                            playlist::SoundcloudPlaylist::from(playlist),
                        )),
                        _ => panic!("something went horribly wrong"),
                    })
                    .collect();
                let folder = provider::ProviderFolder {
                    folders: vec![],
                    items,
                };
                Ok(folder)
            }
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        trace!("search {}", query);
        let client = self.client();
        let results = client
            .tracks()
            .query(Some(query))
            .get()?
            .unwrap_or_else(|| vec![])
            .into_iter()
            .filter(|track| {
                let has_url = track.stream_url.is_some();
                if !has_url {
                    warn!("Track {:?} has no stream url", &track);
                }
                has_url
            })
            .map(SoundcloudTrack::from)
            .map(|track| track.into())
            .collect();
        Ok(results)
    }

    fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        ensure!(
            uri.starts_with(TRACK_URI_PREFIX),
            "Invalid Uri: {}",
            uri
        );
        let id = &uri[TRACK_URI_PREFIX.len()..];
        let id = usize::from_str(id)?;
        let client = self.client();
        let track = client
            .tracks()
            .id(id)
            .get()
            .ok()
            .map(SoundcloudTrack::from)
            .map(Track::from);
        Ok(track)
    }

    fn resolve_album(&self, _uri: &str) -> Result<Option<Album>, Error> {
        Ok(None)
    }

    fn stream_url(&self, track: &Track) -> Result<String, Error> {
        if track.provider == provider::Provider::Soundcloud {
            if let rustic::library::MetaValue::String(stream_url) =
                track.meta.get(meta::META_SOUNDCLOUD_STREAM_URL).unwrap()
            {
                return Ok(format!(
                    "{}?client_id={}",
                    stream_url,
                    self.client_id.clone()
                ));
            }

            return Err(format_err!(
                "Can't get stream url from track, meta value incompatible"
            ));
        }

        Err(format_err!("Invalid provider: {:?}", track.provider))
    }

    fn cover_art(&self, track: &Track) -> Result<Option<provider::CoverArt>, Error> {
        let url = track
            .meta
            .get(meta::META_SOUNDCLOUD_COVER_ART_URL)
            .map(|value| match value {
                MetaValue::String(url) => url.clone(),
                _ => unreachable!(),
            })
            .map(|url| url.into());

        Ok(url)
    }

    fn resolve_share_url(&self, url: url::Url) -> Result<Option<provider::InternalUri>, Error> {
        if url.domain() != Some("soundcloud.com") {
            return Ok(None)
        }
        let client = self.client();
        let url = client.resolve(url.as_str())?;
        let path = url.path();
        if let Some(captures) = SOUNDCLOUD_RESOLVE_REGEX.captures(path) {
            let entity_type = &captures[1];
            let id = &captures[2];
            let entity = match entity_type {
                "tracks" => Some(provider::InternalUri::Track(format!("{}{}", TRACK_URI_PREFIX, id))),
                "playlists" => Some(provider::InternalUri::Playlist(format!("soundcloud://playlist/{}", id))),
                "users" => Some(provider::InternalUri::Artist(format!("soundcloud://user/{}", id))),
                _ => None
            };
            Ok(entity)
        }else {
            Ok(None)
        }
    }
}
