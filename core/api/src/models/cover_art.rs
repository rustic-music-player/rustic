use futures::stream::BoxStream;

pub enum CoverArtModel {
    Data {
        data: BoxStream<'static, Vec<u8>>,
        mime_type: String,
    },
    Url(String),
}
