macro_rules! cstr {
    ($ptr:expr) => {
        ::std::ffi::CString::new($ptr).unwrap().into_raw() as _
    }
}
