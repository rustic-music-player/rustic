use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use rustic_api::models::ArtistModel;
use crate::FutureExt;

#[derive(Debug, Serialize)]
pub struct MpdArtist {
    #[serde(rename = "Artist")]
    artist: String,
}

impl From<ArtistModel> for MpdArtist {
    fn from(artist: ArtistModel) -> Self {
        Self {
            artist: artist.name,
        }
    }
}

pub struct ListArtistCommand {}

impl ListArtistCommand {
    pub fn new() -> ListArtistCommand {
        ListArtistCommand {}
    }
}

impl MpdCommand<Vec<MpdArtist>> for ListArtistCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Vec<MpdArtist>, Error>> {
        async move {
            let artists = client.get_artists().await?;
            let artists: Vec<MpdArtist> = artists
                .into_iter()
                .map(MpdArtist::from)
                .collect();

            Ok(artists)
        }.boxed()
    }
}
