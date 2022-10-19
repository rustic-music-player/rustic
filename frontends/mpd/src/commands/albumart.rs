use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::BoxFuture;
use futures::StreamExt;
use rustic_api::ApiClient;
use rustic_api::cursor::Cursor;
use rustic_api::models::CoverArtModel;
use crate::FutureExt;

pub struct AlbumArtCommand {
    uri: String,
    offset: u32,
}

impl AlbumArtCommand {
    pub fn new(uri: String, offset: u32) -> Self {
        Self {
            uri,
            offset,
        }
    }
}

pub struct AlbumArt {
    pub total_size: usize,
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

impl MpdCommand<AlbumArt> for AlbumArtCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<AlbumArt, Error>> {
        async move {
            let thumbnail = client.get_thumbnail(Cursor::Track(self.uri.clone())).await?;
            match thumbnail {
                Some(CoverArtModel::Data { data, mime_type }) => {
                    let bytes: Vec<Vec<u8>> = data.collect().await;
                    let bytes: Vec<_> = bytes.into_iter().flatten().collect();
                    let total_bytes = bytes.len();
                    let bytes: Vec<_> = bytes.into_iter().skip(self.offset as usize).collect();

                    Ok(AlbumArt {
                        total_size: total_bytes,
                        bytes,
                        mime_type,
                    })
                },
                Some(CoverArtModel::Url(url)) => {
                    todo!()
                },
                None => Err(failure::format_err!("Missing thumbnail"))
            }
        }.boxed()
    }
}
