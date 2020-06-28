macro_rules! cstr {
    ($ptr:expr) => {
        ::std::ffi::CString::new($ptr).unwrap().into_raw() as _
    }
}

macro_rules! optional_cstr {
    ($ptr:expr) => {
        $ptr.map(|string| cstr!(string)).unwrap_or_else(::std::ptr::null)
    }
}

macro_rules! nested_optional {
    ($ptr:expr, $target:ty) => {
        $ptr.map(<$target>::from).map(|model| Box::into_raw(Box::new(model)) as _).unwrap_or_else(::std::ptr::null)
    }
}
