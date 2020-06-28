use rustic_api::models::*;

pub use self::artist::*;
pub use self::album::*;
pub use self::player::*;
pub use self::playlist::*;
pub use self::track::*;

mod artist;
mod album;
mod player;
mod playlist;
mod track;


#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub struct FFISearchResults;

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub struct FFIExtensionModel;

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
    Idle
}

impl From<SearchResults> for FFISearchResults {
    fn from(_: SearchResults) -> Self {
        FFISearchResults
    }
}

impl From<ExtensionModel> for FFIExtensionModel {
    fn from(_: ExtensionModel) -> Self {
        FFIExtensionModel
    }
}

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
