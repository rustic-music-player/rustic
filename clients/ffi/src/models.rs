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
pub struct FFIAggregatedTrack;
#[repr(C)]
pub struct FFIAggregatedAlbum;
#[repr(C)]
pub struct FFIAggregatedArtist;

#[repr(C)]
pub struct FFIProviderItemTypeModel;

#[repr(C)]
pub struct FFIProviderAuthenticationState;

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
