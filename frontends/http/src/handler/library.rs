use std::sync::Arc;

use failure::Error;

use crate::cursor::from_cursor;
use rustic_core::provider::CoverArt;
use rustic_core::{Rustic, SingleQuery};

pub fn get_coverart_for_track(
    cursor: &str,
    rustic: &Arc<Rustic>,
) -> Result<Option<CoverArt>, Error> {
    let uri = from_cursor(cursor)?;
    let query = SingleQuery::uri(uri);
    let track = rustic.query_track(query)?;

    if let Some(track) = track {
        let cover_art = rustic.cover_art(&track)?;

        Ok(cover_art)
    } else {
        Ok(None)
    }
}
