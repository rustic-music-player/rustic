use std::str::from_utf8;

pub fn to_cursor<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode(input)
}

pub fn from_cursor(cursor: &str) -> Result<String, failure::Error> {
    let uri = base64::decode(cursor)?;
    let uri = from_utf8(&uri)?;

    Ok(uri.to_string())
}
