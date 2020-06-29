use std::ffi::CStr;

use libc::*;

use crate::error::FFIError;

macro_rules! cstr {
    ($ptr:expr) => {
        ::std::ffi::CString::new($ptr).unwrap().into_raw() as _
    };
}

macro_rules! optional_cstr {
    ($ptr:expr) => {
        $ptr.map(|string| cstr!(string))
            .unwrap_or_else(::std::ptr::null)
    };
}

macro_rules! optional_number {
    ($ptr:expr) => {
        $ptr.unwrap_or_default()
    };
}

macro_rules! nested_optional {
    ($ptr:expr, $target:ty) => {
        $ptr.map(<$target>::from)
            .map(|model| Box::into_raw(Box::new(model)) as _)
            .unwrap_or_else(::std::ptr::null)
    };
}

pub(crate) unsafe fn to_str<'s>(input: *const c_char) -> Result<Option<&'s str>, FFIError> {
    if input.is_null() {
        return Ok(None);
    }

    let raw = CStr::from_ptr(input);

    let query = raw.to_str()?;

    Ok(Some(query))
}
