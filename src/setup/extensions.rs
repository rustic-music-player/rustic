use failure::Error;

use rustic_extension_api::{ExtensionManager, ExtensionManagerBuilder};

use crate::options::CliOptions;
use crate::{config::Config, options};
use std::path::Path;

pub(crate) async fn load_extensions(
    options: &options::CliOptions,
    config: &Config,
) -> Result<ExtensionManager, Error> {
    let mut manager = ExtensionManagerBuilder::default();

    let path = get_load_path(options, config);
    if let Some(path) = path {
        manager
            .load_dir(path, &config.extensions.extensions)
            .await?;
    }

    Ok(manager.build())
}

fn get_load_path<'a>(options: &'a CliOptions, config: &'a Config) -> Option<&'a Path> {
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
    paths.into_iter().find(|path| path.exists())
}
