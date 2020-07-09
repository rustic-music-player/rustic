use rustic_api::models::*;

rustic_ffi_client_macros::client_models!();

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub struct FFIOpenResultModel;

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub struct FFICoverArtModel;

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub enum FFISyncStateModel {
    Idle,
}

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub enum FFIProviderTypeModel {
    Idle,
}

#[derive(Debug)]
#[repr(C)]
pub enum FFIRepeatModeModel {
    None,
    Single,
    All,
}

#[derive(Debug)]
#[repr(C)]
pub enum FFISyncItemStateModel {
    Idle,
    Syncing,
    Done,
    Error,
}

#[repr(C)]
pub enum FFIAggregatedTrack {
    Single(FFITrackModel),
    Multi(FFITrackCollection),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum FFIMetaValueModel {
    Bool(bool),
    String(String),
    Float(f64),
    Int(u64),
}

impl From<MetaValueModel> for FFIMetaValueModel {
    fn from(value: MetaValueModel) -> Self {
        match value {
            MetaValueModel::String(string) => FFIMetaValueModel::String(string),
            MetaValueModel::Int(int) => FFIMetaValueModel::Int(int),
            MetaValueModel::Float(float) => FFIMetaValueModel::Float(float),
            MetaValueModel::Bool(bool) => FFIMetaValueModel::Bool(bool),
        }
    }
}

impl From<AggregatedTrack> for FFIAggregatedTrack {
    fn from(track: AggregatedTrack) -> Self {
        match track {
            AggregatedTrack::Single(track) => FFIAggregatedTrack::Single(track.into()),
            AggregatedTrack::Multi(track) => FFIAggregatedTrack::Multi(track.into()),
        }
    }
}

#[repr(C)]
pub enum FFIAggregatedAlbum {
    Single(FFIAlbumModel),
    Multi(FFIAlbumCollection),
}

impl From<AggregatedAlbum> for FFIAggregatedAlbum {
    fn from(album: AggregatedAlbum) -> Self {
        match album {
            AggregatedAlbum::Single(album) => FFIAggregatedAlbum::Single(album.into()),
            AggregatedAlbum::Multi(album) => FFIAggregatedAlbum::Multi(album.into()),
        }
    }
}

#[repr(C)]
pub enum FFIAggregatedArtist {
    Single(FFIArtistModel),
    Multi(FFIArtistCollection),
}

impl From<AggregatedArtist> for FFIAggregatedArtist {
    fn from(artist: AggregatedArtist) -> Self {
        match artist {
            AggregatedArtist::Single(artist) => FFIAggregatedArtist::Single(artist.into()),
            AggregatedArtist::Multi(artist) => FFIAggregatedArtist::Multi(artist.into()),
        }
    }
}

#[repr(C)]
pub struct FFIProviderItemTypeModel;

#[repr(C)]
pub struct FFIProviderStateModel;

impl From<OpenResultModel> for FFIOpenResultModel {
    fn from(_: OpenResultModel) -> Self {
        FFIOpenResultModel
    }
}

impl From<CoverArtModel> for FFICoverArtModel {
    fn from(_: CoverArtModel) -> Self {
        FFICoverArtModel
    }
}

impl From<SyncStateModel> for FFISyncStateModel {
    fn from(_: SyncStateModel) -> Self {
        FFISyncStateModel::Idle
    }
}

impl From<RepeatModeModel> for FFIRepeatModeModel {
    fn from(model: RepeatModeModel) -> Self {
        match model {
            RepeatModeModel::None => FFIRepeatModeModel::None,
            RepeatModeModel::Single => FFIRepeatModeModel::Single,
            RepeatModeModel::All => FFIRepeatModeModel::All,
        }
    }
}

impl From<SyncItemStateModel> for FFISyncItemStateModel {
    fn from(model: SyncItemStateModel) -> Self {
        match model {
            SyncItemStateModel::Idle => FFISyncItemStateModel::Idle,
            SyncItemStateModel::Syncing => FFISyncItemStateModel::Syncing,
            SyncItemStateModel::Done => FFISyncItemStateModel::Done,
            SyncItemStateModel::Error => FFISyncItemStateModel::Error,
        }
    }
}

#[repr(C)]
pub struct FFIRatingModel;
