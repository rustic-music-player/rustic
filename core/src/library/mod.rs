mod rating;
mod album;
mod artist;
mod library;
mod meta;
mod playlist;
pub mod queries;
mod track;

pub use self::album::Album;
pub use self::artist::Artist;
pub use self::library::{Library, SearchResults, SharedLibrary};
pub use self::meta::MetaValue;
pub use self::playlist::Playlist;
pub use self::queries::*;
pub use self::track::Track;
pub use self::rating::Rating;

pub trait Identifiable {
    fn get_uri(&self) -> String;
    fn get_id(&self) -> Option<usize>;

    fn get_identifier(&self) -> SingleQueryIdentifier {
        if let Some(id) = self.get_id() {
            SingleQueryIdentifier::Id(id)
        } else {
            SingleQueryIdentifier::Uri(self.get_uri())
        }
    }
}
