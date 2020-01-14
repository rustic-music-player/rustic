use rustic_core::Provider;
use rustic_core::provider::{ProviderFolder, ProviderItem, ProviderItemType};
use viewmodels::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};

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
    pub fn new(folder: ProviderFolder) -> Self {
        ProviderFolderModel {
            folders: folder.folders,
            items: folder.items.iter()
                .map(|item| ProviderItemModel::new(item))
                .collect()
        }
    }
}

impl ProviderItemModel {
    fn new(item: &ProviderItem) -> Self {
        ProviderItemModel {
            label: item.label.clone(),
            data: ProviderItemTypeModel::new(&item.data)
        }
    }
}

impl ProviderItemTypeModel {
    fn new(item_type: &ProviderItemType) -> Self {
        match item_type {
            ProviderItemType::Track(track) => ProviderItemTypeModel::Track(TrackModel::new(track.clone())),
            ProviderItemType::Album(album) => ProviderItemTypeModel::Album(AlbumModel::new(album.clone())),
            ProviderItemType::Artist(artist) => ProviderItemTypeModel::Artist(ArtistModel::new(artist.clone())),
            ProviderItemType::Playlist(playlist) => ProviderItemTypeModel::Playlist(PlaylistModel::new(playlist.clone())),
        }
    }
}