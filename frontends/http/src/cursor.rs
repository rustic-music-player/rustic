use std::str::from_utf8;

use base64::{decode, encode};

pub(crate) fn to_cursor<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    encode(input)
}

pub(crate) fn from_cursor(cursor: &str) -> Result<String, failure::Error> {
    let uri = decode(cursor)?;
    let uri = from_utf8(&uri)?;

    Ok(uri.to_string())
}
