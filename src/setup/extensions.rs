use crate::{options, config::Config};
use std::path::Path;
use rustic_core::extension::HostedExtension;
use failure::Error;

pub(crate) fn load_extensions(
    options: &options::CliOptions,
    config: &Config,
) -> Result<Vec<HostedExtension>, Error> {
    let mut paths = vec![
        Path::new("target/debug"),
        Path::new("target/release"),
        Path::new("extensions"),
    ];
    if let Some(ref path) = config.extensions.path {
        paths.insert(0, Path::new(path));
    }
    if let Some(ref path) = options.extensions_path {
        paths.insert(0, Path::new(path));
    }
    let path = paths.iter().find(|path| path.exists());
    if let Some(path) = path {
        let extensions = rustic_core::extension::load_extensions(path)?;
        Ok(extensions)
    } else {
        Ok(Vec::new())
    }
}
