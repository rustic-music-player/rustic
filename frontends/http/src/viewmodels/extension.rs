use rustic_core::extension::HostedExtension;

#[derive(Debug, Clone, Serialize)]
pub struct ExtensionModel {
    name: String,
    id: String,
    version: String,
    enabled: bool
}

impl From<&HostedExtension> for ExtensionModel {
    fn from(extension: &HostedExtension) -> Self {
        ExtensionModel {
            name: extension.name.clone(),
            id: extension.id.clone(),
            version: extension.version.clone(),
            enabled: true
        }
    }
}