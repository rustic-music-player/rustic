use crate::{config::Config, options};
use failure::Error;
use rustic_extension_api::*;

pub(crate) fn load_extensions(
    _options: &options::CliOptions,
    config: &Config,
) -> Result<ExtensionManager, Error> {
    let mut manager = ExtensionManagerBuilder::default();
    #[cfg(feature = "extensions")]
    {
        load_extension::<rustic_party_mode_extension::PartyModeExtension>(&mut manager, config);
    }

    Ok(manager.build())
}

fn load_extension<T>(manager: &mut ExtensionManagerBuilder, config: &Config)
where
    T: ExtensionLibrary,
{
    let metadata = T::metadata();
    let config = config
        .extensions
        .extensions
        .get(&metadata.id)
        .cloned()
        .unwrap_or_default();
    manager.load::<T>(config);
}
