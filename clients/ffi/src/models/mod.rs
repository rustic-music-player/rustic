use rustic_api::models::*;

pub mod player;

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
