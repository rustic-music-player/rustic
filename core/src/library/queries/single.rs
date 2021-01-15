use crate::library::LibraryItemIdentifier;

use super::{LibraryQueryJoins, QueryJoins};

#[derive(Default, Debug, Clone)]
pub struct SingleQuery {
    pub identifier: LibraryItemIdentifier,
    pub joins: LibraryQueryJoins,
}

impl From<LibraryItemIdentifier> for SingleQuery {
    fn from(identifier: LibraryItemIdentifier) -> Self {
        SingleQuery {
            identifier,
            ..SingleQuery::default()
        }
    }
}

impl From<usize> for SingleQuery {
    fn from(id: usize) -> Self {
        LibraryItemIdentifier::Id(id).into()
    }
}

impl From<String> for SingleQuery {
    fn from(query: String) -> Self {
        LibraryItemIdentifier::Uri(query).into()
    }
}

impl SingleQuery {
    pub fn id(id: usize) -> Self {
        SingleQuery {
            identifier: LibraryItemIdentifier::Id(id),
            ..SingleQuery::default()
        }
    }

    pub fn uri(uri: String) -> Self {
        SingleQuery {
            identifier: LibraryItemIdentifier::Uri(uri),
            ..SingleQuery::default()
        }
    }
}

impl QueryJoins for SingleQuery {
    fn join_all(&mut self) -> &mut Self {
        self.joins = LibraryQueryJoins::ALL;
        self
    }

    fn join_tracks(&mut self) -> &mut Self {
        self.joins |= LibraryQueryJoins::TRACK;
        self
    }

    fn join_albums(&mut self) -> &mut Self {
        self.joins |= LibraryQueryJoins::ALBUM;
        self
    }

    fn join_artists(&mut self) -> &mut Self {
        self.joins |= LibraryQueryJoins::ARTIST;
        self
    }
}
