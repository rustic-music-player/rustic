use super::{LibraryQueryJoins, QueryJoins};

#[derive(Default, Debug, Clone)]
pub struct SingleQuery {
    pub identifier: SingleQueryIdentifier,
    pub joins: LibraryQueryJoins,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleQueryIdentifier {
    Id(usize),
    Uri(String),
}

impl From<SingleQueryIdentifier> for SingleQuery {
    fn from(identifier: SingleQueryIdentifier) -> Self {
        SingleQuery {
            identifier,
            ..SingleQuery::default()
        }
    }
}

impl From<usize> for SingleQuery {
    fn from(id: usize) -> Self {
        SingleQueryIdentifier::Id(id).into()
    }
}

impl From<String> for SingleQuery {
    fn from(query: String) -> Self {
        SingleQueryIdentifier::Uri(query).into()
    }
}

impl Default for SingleQueryIdentifier {
    fn default() -> SingleQueryIdentifier {
        SingleQueryIdentifier::Id(0)
    }
}

impl SingleQuery {
    pub fn id(id: usize) -> Self {
        SingleQuery {
            identifier: SingleQueryIdentifier::Id(id),
            ..SingleQuery::default()
        }
    }

    pub fn uri(uri: String) -> Self {
        SingleQuery {
            identifier: SingleQueryIdentifier::Uri(uri),
            ..SingleQuery::default()
        }
    }
    
    pub fn matches(&self, identifier: SingleQueryIdentifier) -> bool {
        self.identifier == identifier
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
