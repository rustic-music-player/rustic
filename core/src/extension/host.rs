use crate::extension::commands::{ExtensionCommands, ExtensionResponses, Hook};
use crate::extension::ClientExtension;
use log::{debug, error, info};
use std::fs::{read_dir, DirEntry};
use std::io::Stdin;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::Mutex;

pub fn load_extensions(path: &Path) -> Result<Vec<HostedExtension>, failure::Error> {
    let extensions = read_dir(path)?
        .into_iter()
        .filter(|file| file.is_ok())
        .map(|file| file.unwrap())
        .filter(|file| {
            file.file_type()
                .map(|file_type| file_type.is_file())
                .unwrap_or(false)
        })
        .filter(|file| {
            file.file_name()
                .into_string()
                .unwrap()
                .ends_with("extension")
        })
        .map(load_extension)
        .filter(|(path, extension)| {
            let is_err = extension.is_err();
            if is_err {
                error!(
                    "Error loading extension {}: {:?}",
                    path,
                    extension.as_ref().unwrap_err()
                );
            }
            is_err
        })
        .map(|(_, extension)| extension.unwrap())
        .collect();

    Ok(extensions)
}

fn load_extension(file: DirEntry) -> (String, Result<HostedExtension, failure::Error>) {
    let file_path = file.path().to_str().unwrap().to_string();
    debug!("Loading extension {}", file_path);
    let extension = HostedExtension::new(file_path.clone());
    (file_path, extension)
}

#[derive(Debug)]
pub struct HostedExtension {
    process: Arc<Mutex<Child>>,
    id: String,
    name: String,
    version: String,
    hooks: Vec<Hook>,
}

impl HostedExtension {
    fn new(path: String) -> Result<HostedExtension, failure::Error> {
        let mut process = Command::new(&path)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        let stdin = process.stdin.as_mut().unwrap();
        bincode::serialize_into(stdin, &ExtensionCommands::Load)?;
        let stdout = process.stdout.as_mut().unwrap();
        let response: ExtensionResponses = bincode::deserialize_from(stdout)?;
        let metadata = if let ExtensionResponses::Load(metadata) = response {
            Some(metadata)
        } else {
            None
        }
        .unwrap();
        info!("Loaded Extension: {} v{}", metadata.name, metadata.version);
        debug!("> Hooks: {:?}", metadata.hooks);
        Ok(HostedExtension {
            process: Arc::new(Mutex::new(process)),
            id: metadata.id,
            name: metadata.name,
            version: metadata.version,
            hooks: metadata.hooks,
        })
    }

    fn run_command(
        &self,
        command: ExtensionCommands,
    ) -> Result<ExtensionResponses, failure::Error> {
        let mut process = self.process.lock().unwrap();
        let stdin = process.stdin.as_mut().unwrap();
        bincode::serialize_into(stdin, &command)?;
        let stdout = process.stdout.as_mut().unwrap();
        let response = bincode::deserialize_from(stdout)?;

        Ok(response)
    }
}
