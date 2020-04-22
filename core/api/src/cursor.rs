pub fn to_cursor<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode(input)
}
