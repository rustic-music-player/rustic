use std::collections::HashMap;

use rustic_extension_api::*;

#[derive(Debug)]
pub struct PartyModeExtension;

impl ExtensionLibrary for PartyModeExtension {
    fn new(_config: HashMap<String, ExtensionConfigValue>) -> Self {
        PartyModeExtension
    }

    fn metadata() -> ExtensionMetadata {
        ExtensionMetadata {
            id: String::from("party-mode"),
            name: String::from("Party Mode"),
            version: crate_version!(),
        }
    }
}

impl Extension for PartyModeExtension {}

impl ExtensionApi for PartyModeExtension {}

host_extension!(PartyModeExtension);
