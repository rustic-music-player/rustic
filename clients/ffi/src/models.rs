use rustic_api::models::*;

rustic_ffi_client_macros::client_models!();

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

#[derive(Debug)]
#[repr(C)]
// TODO: add fields
pub enum FFIProviderTypeModel {
    Idle
}

#[derive(Debug)]
#[repr(C)]
pub enum FFIRepeatModeModel {
    None,
    Single,
    All
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

impl From<RepeatModeModel> for FFIRepeatModeModel {
    fn from(model: RepeatModeModel) -> Self {
        match model {
            RepeatModeModel::None => FFIRepeatModeModel::None,
            RepeatModeModel::Single => FFIRepeatModeModel::Single,
            RepeatModeModel::All => FFIRepeatModeModel::All
        }
    }
}
