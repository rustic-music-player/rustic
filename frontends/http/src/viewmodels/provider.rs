use rustic_core::{Provider, Rustic};
use rustic_core::provider::{ProviderFolder, ProviderItem, ProviderItemType};
use viewmodels::{TrackModel, AlbumModel, ArtistModel, PlaylistModel};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct ProviderModel {
    pub title: String,
    pub provider: Provider,
    pub explore: ProviderFolder
}

#[derive(Clone, Debug, Serialize)]
pub struct ProviderFolderModel {
    pub folders: Vec<String>,
    pub items: Vec<ProviderItemModel>
}

#[derive(Clone, Debug, Serialize)]
pub struct ProviderItemModel {
    pub label: String,
    pub data: ProviderItemTypeModel,
}

#[derive(Clone, Debug, Serialize)]
pub enum ProviderItemTypeModel {
    Track(TrackModel),
    Album(AlbumModel),
    Artist(ArtistModel),
    Playlist(PlaylistModel),
}

impl ProviderFolderModel {
    pub fn new(folder: ProviderFolder, app: &Arc<Rustic>) -> Self {
        ProviderFolderModel {
            folders: folder.folders,
            items: folder.items.iter()
                .map(|item| ProviderItemModel::new(item, app))
                .collect()
        }
    }
}

impl ProviderItemModel {
    fn new(item: &ProviderItem, app: &Arc<Rustic>) -> Self {
        ProviderItemModel {
            label: item.label.clone(),
            data: ProviderItemTypeModel::new(&item.data, app)
        }
    }
}

impl ProviderItemTypeModel {
    fn new(item_type: &ProviderItemType, app: &Arc<Rustic>) -> Self {
        match item_type {
            ProviderItemType::Track(track) => ProviderItemTypeModel::Track(TrackModel::new(track.clone(), app)),
            ProviderItemType::Album(album) => ProviderItemTypeModel::Album(AlbumModel::new(album.clone(), app)),
            ProviderItemType::Artist(artist) => ProviderItemTypeModel::Artist(ArtistModel::new(artist.clone(), app)),
            ProviderItemType::Playlist(playlist) => ProviderItemTypeModel::Playlist(PlaylistModel::new(playlist.clone(), app)),
        }
    }
}