use crate::plugin::ExtensionCommand;
use crate::ExtensionConfigValue;
use failure::format_err;
use std::collections::HashMap;
use std::ffi::OsStr;
use tokio::sync::mpsc;

pub struct ExtensionHost {
    extension: mpsc::Sender<ExtensionCommand>,
    task: tokio::task::JoinHandle<Option<u8>>,
}

impl ExtensionHost {
    pub fn new(mut plugin: Box<dyn ExtensionPlugin>) -> Self {
        let (tx, mut rx) = mpsc::channel(10);
        ExtensionHost {
            extension: tx,
            task: tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if let Some(status) = plugin.handle_message(message).await {
                        return Some(status);
                    }
                }
                None
            }),
        }
    }

    pub async fn send(&mut self, message: ExtensionCommand) {
        self.extension.send(message).await;
    }
}

#[async_trait::async_trait]
pub trait ExtensionPlugin: Sync + Send + 'static {
    async fn handle_message(&mut self, message: ExtensionCommand) -> Option<u8>;
}

pub fn construct_plugin(
    path: impl AsRef<OsStr>,
    args: &HashMap<String, HashMap<String, ExtensionConfigValue>>,
) -> Result<Box<dyn ExtensionPlugin>, failure::Error> {
    let lib = libloading::Library::new(path)?;
    let mut instance = std::mem::MaybeUninit::zeroed();
    Ok(unsafe {
        lib.get::<FfiPluginInit>(b"plugin_constructor")?(
            instance.as_mut_ptr(),
            Box::into_raw(Box::new(args)),
        );
        if ((*instance.as_ptr()).as_ref() as *const dyn ExtensionPlugin).is_null() {
            return Err(format_err!("Can't construct extension"));
        }
        instance.assume_init()
    })
}

/// Inserts a plugin into an uninitialized pointer, preventing the drop on the uninitialized memory that would happen with a simple assignment
pub fn insert_instance(ptr: *mut Box<dyn ExtensionPlugin>, mut plugin: Box<dyn ExtensionPlugin>) {
    unsafe { std::mem::swap(&mut plugin, &mut *ptr) };
    std::mem::forget(plugin);
}

pub type FfiPluginInit = unsafe extern "C" fn(
    *mut Box<dyn ExtensionPlugin>,
    *mut &HashMap<String, HashMap<String, ExtensionConfigValue>>,
);
