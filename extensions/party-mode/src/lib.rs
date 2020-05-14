use rustic_extension_api::*;

#[derive(Debug)]
pub struct PartyModeExtension {

}

impl ExtensionLibrary for PartyModeExtension {
    fn new() -> Box<dyn Extension> {
        let extension = PartyModeExtension {};
        Box::new(extension)
    }
}

impl Extension for PartyModeExtension {
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            id: String::from("party-mode"),
            name: String::from("Party Mode"),
            version: crate_version!()
        }
    }
}

impl ExtensionApi for PartyModeExtension {

}
