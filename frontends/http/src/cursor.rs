use std::str::from_utf8;

pub(crate) fn from_cursor(cursor: &str) -> Result<String, failure::Error> {
    let uri = urlencoding::decode(cursor)?;
    let uri = base64::decode(&uri)?;
    let uri = from_utf8(&uri)?;

    Ok(uri.to_string())
}
