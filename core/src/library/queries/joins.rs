use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct LibraryQueryJoins: u32 {
        const TRACK =  0b0000_0001;
        const ALBUM =  0b0000_0010;
        const ARTIST = 0b0000_0100;
        const ALL = Self::TRACK.bits | Self::ALBUM.bits | Self::ARTIST.bits;
    }
}

impl LibraryQueryJoins {
    pub fn has_tracks(self) -> bool {
        self.contains(LibraryQueryJoins::TRACK)
    }

    pub fn has_albums(self) -> bool {
        self.contains(LibraryQueryJoins::ALBUM)
    }

    pub fn has_artists(self) -> bool {
        self.contains(LibraryQueryJoins::ARTIST)
    }
}

pub trait QueryJoins {
    fn join_all(&mut self) -> &mut Self;
    fn join_tracks(&mut self) -> &mut Self;
    fn join_albums(&mut self) -> &mut Self;
    fn join_artists(&mut self) -> &mut Self;
}

#[cfg(test)]
mod tests {
    use super::LibraryQueryJoins;

    #[test]
    fn has_tracks_should_return_true_when_tracks_bit_is_set() {
        let query = LibraryQueryJoins::TRACK;

        let result = query.has_tracks();

        assert_eq!(result, true);
    }

    #[test]
    fn has_albums_should_return_true_when_tracks_bit_is_set() {
        let query = LibraryQueryJoins::ALBUM;

        let result = query.has_albums();

        assert_eq!(result, true);
    }

    #[test]
    fn has_artists_should_return_true_when_tracks_bit_is_set() {
        let query = LibraryQueryJoins::ARTIST;

        let result = query.has_artists();

        assert_eq!(result, true);
    }
}
