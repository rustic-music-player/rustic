use crate::{options, config::Config};
use rustic_extension_api::*;
use failure::Error;

pub(crate) fn load_extensions(
    _options: &options::CliOptions,
    _config: &Config,
) -> Result<ExtensionManager, Error> {
    let mut manager = ExtensionManagerBuilder::default();
    #[cfg(feature = "extensions")]
    {
        manager.load::<rustic_party_mode_extension::PartyModeExtension>();
    }

    Ok(manager.build())
}
