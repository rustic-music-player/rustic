use crate::plugin::ExtensionCommand;
use crate::ExtensionConfigValue;
use failure::format_err;
use rustic_queue::{broadcast, Sender};
use std::collections::HashMap;
use std::ffi::OsStr;

pub struct ExtensionHost {
    extension: Sender<ExtensionCommand>,
    task: tokio::task::JoinHandle<Option<u8>>,
    library: libloading::Library,
}

impl ExtensionHost {
    pub fn new((library, mut plugin): (libloading::Library, Box<dyn ExtensionPlugin>)) -> Self {
        let (tx, rx) = broadcast();
        ExtensionHost {
            library,
            extension: tx,
            task: tokio::spawn(async move {
                while let Ok(message) = rx.recv_async().await {
                    log::trace!("delegating message to plugin handler {:?}", message);
                    if let Some(status) = plugin.handle_message(message).await {
                        return Some(status);
                    }
                }
                None
            }),
        }
    }

    pub async fn send(&mut self, message: ExtensionCommand) {
        self.extension.send_async(message).await;
    }
}

#[async_trait::async_trait]
pub trait ExtensionPlugin: Sync + Send + 'static {
    async fn handle_message(&mut self, message: ExtensionCommand) -> Option<u8>;
}

pub fn construct_plugin(
    path: impl AsRef<OsStr>,
    args: &HashMap<String, HashMap<String, ExtensionConfigValue>>,
) -> Result<(libloading::Library, Box<dyn ExtensionPlugin>), failure::Error> {
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
        (lib, instance.assume_init())
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
