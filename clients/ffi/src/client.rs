use std::ptr::NonNull;

use rustic_api::client::RusticApiClient;

use std::sync::Arc;

#[repr(C)]
pub struct RusticClientHandle {
    _private: [u8; 0],
}

pub(crate) struct Client(NonNull<Arc<Box<dyn RusticApiClient>>>);

unsafe impl std::marker::Send for Client {}

unsafe impl std::marker::Sync for Client {}

impl Client {
    pub(crate) unsafe fn new(client: Box<dyn RusticApiClient>) -> Self {
        Client(NonNull::new_unchecked(Box::into_raw(Box::new(Arc::new(
            client,
        )))))
    }

    pub(crate) unsafe fn to_ptr(&self) -> *mut RusticClientHandle {
        self.0.as_ptr().cast()
    }

    pub(crate) unsafe fn from_ptr(client: *mut RusticClientHandle) -> Self {
        let client = client.cast();

        Client(NonNull::new_unchecked(client))
    }

    pub(crate) fn get_client(&self) -> &Arc<Box<dyn RusticApiClient>> {
        unsafe { self.0.as_ref() }
    }
}
