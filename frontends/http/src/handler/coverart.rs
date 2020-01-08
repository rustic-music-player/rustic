use std::sync::Arc;
use rustic_core::{Rustic, Track};
use rustic_core::provider::CoverArt;
use failure::Error;

pub fn get_coverart(track: &Track, rustic: &Arc<Rustic>) -> Result<Option<CoverArt>, Error> {
    let cover_art = rustic.cover_art(track)?;
    Ok(cover_art)
}
