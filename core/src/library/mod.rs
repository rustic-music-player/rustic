pub use self::album::Album;
pub use self::artist::Artist;
pub use self::event::*;
pub use self::library::{Library, SearchResults, SharedLibrary};
pub use self::meta::MetaValue;
pub use self::playlist::Playlist;
pub use self::queries::*;
pub use self::rating::Rating;
pub use self::track::*;

use serde::{Deserialize, Serialize};

mod album;
mod artist;
mod event;
mod library;
mod meta;
mod playlist;
pub mod queries;
mod rating;
mod track;

pub trait Identifiable {
    fn get_uri(&self) -> String;
    fn get_id(&self) -> Option<usize>;

    fn get_identifier(&self) -> LibraryItemIdentifier {
        if let Some(id) = self.get_id() {
            LibraryItemIdentifier::Id(id)
        } else {
            LibraryItemIdentifier::Uri(self.get_uri())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum LibraryItemIdentifier {
    Id(usize),
    Uri(String),
}

impl Default for LibraryItemIdentifier {
    fn default() -> LibraryItemIdentifier {
        LibraryItemIdentifier::Id(0)
    }
}

impl std::fmt::Display for LibraryItemIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LibraryItemIdentifier::Id(id) => write!(f, "id:{}", id),
            LibraryItemIdentifier::Uri(uri) => write!(f, "uri:{}", uri),
        }
    }
}

impl From<String> for LibraryItemIdentifier {
    fn from(uri: String) -> Self {
        LibraryItemIdentifier::Uri(uri)
    }
}

impl From<&str> for LibraryItemIdentifier {
    fn from(uri: &str) -> Self {
        LibraryItemIdentifier::Uri(uri.to_string())
    }
}

impl From<usize> for LibraryItemIdentifier {
    fn from(id: usize) -> Self {
        LibraryItemIdentifier::Id(id)
    }
}

impl<T: Identifiable> PartialEq<T> for LibraryItemIdentifier {
    fn eq(&self, other: &T) -> bool {
        let rhs = other.get_identifier();

        self == &rhs
    }
}
