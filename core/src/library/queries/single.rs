use super::{LibraryQueryJoins, QueryJoins};

#[derive(Default, Debug, Clone)]
pub struct SingleQuery {
    pub identifier: SingleQueryIdentifier,
    pub joins: LibraryQueryJoins,
}

#[derive(Debug, Clone)]
pub enum SingleQueryIdentifier {
    Id(usize),
    Uri(String),
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
