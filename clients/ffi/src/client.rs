use std::ptr::NonNull;
use rustic_api::client::RusticApiClient;
use std::ffi::CStr;
use libc::*;
use crate::error::FFIError;
use crate::models::player::Player;

#[repr(C)]
pub struct RusticClientHandle { _private: [u8; 0] }

pub(crate) struct Client(NonNull<Box<dyn RusticApiClient>>);

unsafe impl std::marker::Send for Client {}

unsafe impl std::marker::Sync for Client {}

pub(crate) unsafe fn to_str<'s>(input: *const c_char) -> Result<Option<&'s str>, FFIError> {
    if input.is_null() {
        return Ok(None);
    }

    let raw = CStr::from_ptr(input);

    let query = raw.to_str()?;

    Ok(Some(query))
}

impl Client {
    pub(crate) unsafe fn new(client: Box<dyn RusticApiClient>) -> Self {
        Client(NonNull::new_unchecked(Box::into_raw(Box::new(client))))
    }

    pub(crate) unsafe fn to_ptr(&self) -> *mut RusticClientHandle {
        self.0.as_ptr().cast()
    }

    pub(crate) unsafe fn from_ptr(client: *mut RusticClientHandle) -> Self {
        let client = client.cast();

        Client(NonNull::new_unchecked(client))
    }

    pub async fn get_player(&self) -> Result<Player, FFIError> {
        let client = unsafe { self.0.as_ref() };

        let res = client.get_player(None).await?;

        Ok(res.unwrap().into())
    }
}
