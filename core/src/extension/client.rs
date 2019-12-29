use std::error::Error;

use crate::extension::commands::{ExtensionCommands, ExtensionMetadata, ExtensionResponses, Hook};
use crate::Track;

#[macro_export]
macro_rules! crate_version {
    () => {
        format!(
            "{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
        )
    };
}

#[derive(Debug)]
pub struct ClientExtension {
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) version: String,
    hooks: Vec<ClientHook>,
}

#[derive(Debug)]
pub struct ClientHook {
    hook: HookType,
}

enum HookType {
    AddToQueue(Box<dyn Fn(Vec<Track>) -> Result<Vec<Track>, Box<dyn Error>>>),
}

impl From<&ClientHook> for Hook {
    fn from(hook: &ClientHook) -> Self {
        match hook.hook {
            HookType::AddToQueue(_) => Hook::AddToQueue,
        }
    }
}

impl std::fmt::Debug for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HookType::AddToQueue")
    }
}

#[derive(Debug)]
pub struct ExtensionBuilder {
    name: String,
    id: String,
    version: String,
    hooks: Vec<ClientHook>,
}

impl ExtensionBuilder {
    pub fn new<I: Into<String>, N: Into<String>, V: Into<String>>(
        id: I,
        name: N,
        version: V,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            hooks: vec![],
        }
    }

    pub fn on_add_to_queue(
        self,
        hook: &'static dyn Fn(Vec<Track>) -> Result<Vec<Track>, Box<dyn Error>>,
    ) -> Self {
        self.add_hook(HookType::AddToQueue(Box::new(hook)))
    }

    fn add_hook(mut self, hook: HookType) -> Self {
        self.hooks.push(ClientHook { hook });
        self
    }

    pub fn build(self) -> ClientExtension {
        ClientExtension {
            id: self.id,
            name: self.name,
            version: self.version,
            hooks: self.hooks,
        }
    }
}

pub fn host_extension(extension: ClientExtension) {
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    //    loop {
    let command: ExtensionCommands = bincode::deserialize_from(&mut stdin).unwrap();
    let response = match command {
        ExtensionCommands::Load => ExtensionResponses::Load(ExtensionMetadata {
            name: extension.name,
            id: extension.id,
            version: extension.version,
            hooks: extension.hooks.iter().map(|hook| hook.into()).collect(),
        }),
    };
    bincode::serialize_into(&mut stdout, &response);
    //    }
}
