use crate::ProviderType;

use super::{LibraryQueryJoins, QueryJoins};

#[derive(Default, Debug, Clone)]
pub struct MultiQuery {
    pub joins: LibraryQueryJoins,
    pub limit: Option<usize>,
    pub providers: Vec<ProviderType>,
}

impl MultiQuery {
    pub fn new() -> MultiQuery {
        MultiQuery::default()
    }

    pub fn limit(&mut self, limit: usize) -> &mut MultiQuery {
        self.limit = Some(limit);
        self
    }

    pub fn with_providers(&mut self, providers: Vec<ProviderType>) -> &mut MultiQuery {
        self.providers = providers;
        self
    }
}

impl QueryJoins for MultiQuery {
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

#[cfg(test)]
mod tests {
    use crate::MultiQuery;

    use super::{LibraryQueryJoins, QueryJoins};

    #[test]
    fn join_all_should_set_bits_for_all_joins() {
        let mut query = MultiQuery::new();
        query.join_all();

        assert_eq!(query.joins, LibraryQueryJoins::ALL);
    }

    #[test]
    fn join_tracks_should_set_additional_bit_for_tracks() {
        let mut query = MultiQuery::new();
        query.joins = LibraryQueryJoins::ALBUM;
        query.join_tracks();

        assert_eq!(
            query.joins,
            LibraryQueryJoins::ALBUM | LibraryQueryJoins::TRACK
        );
    }

    #[test]
    fn join_albums_should_set_bits_for_all_joins() {
        let mut query = MultiQuery::new();
        query.joins = LibraryQueryJoins::ARTIST;
        query.join_albums();

        assert_eq!(
            query.joins,
            LibraryQueryJoins::ARTIST | LibraryQueryJoins::ALBUM
        );
    }

    #[test]
    fn join_artists_should_set_bits_for_all_joins() {
        let mut query = MultiQuery::new();
        query.joins = LibraryQueryJoins::TRACK;
        query.join_artists();

        assert_eq!(
            query.joins,
            LibraryQueryJoins::TRACK | LibraryQueryJoins::ARTIST
        );
    }

    #[test]
    fn joining_all_fields_should_equal_all() {
        let mut query = MultiQuery::new();
        query.join_tracks();
        query.join_albums();
        query.join_artists();

        assert_eq!(query.joins, LibraryQueryJoins::ALL);
    }
}
