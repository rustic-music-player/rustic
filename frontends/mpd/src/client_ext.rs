use failure::Error;
use futures::future::BoxFuture;
use futures::FutureExt;
use rustic_api::ApiClient;
use rustic_api::models::PlaylistModel;

pub trait ClientExt {
    fn get_playlist_by_name<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Result<Option<PlaylistModel>, failure::Error>>;
}

impl ClientExt for ApiClient {
    fn get_playlist_by_name<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Result<Option<PlaylistModel>, Error>> {
        async move {
            let playlists = self.get_playlists(None).await?;
            let playlist = playlists
                .into_iter()
                .find(|playlist| playlist.title == name);

            Ok(playlist)
        }.boxed()
    }
}
