use std::sync::Arc;

use failure::Error;

use rustic_core::Rustic;
use rustic_api::models::*;

pub fn open(url: String, rustic: &Arc<Rustic>) -> Result<Option<OpenResultModel>, Error> {
    let internal_url = rustic.resolve_share_url(url)?;
    let internal_url = internal_url.map(OpenResultModel::from);

    Ok(internal_url)
}
