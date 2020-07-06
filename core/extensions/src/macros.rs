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

#[macro_export]
macro_rules! host_extension {
    ($extension:ty) => {
        #[no_mangle]

        pub unsafe extern "C" fn plugin_constructor(
            plugin: *mut Box<dyn rustic_extension_api::ExtensionPlugin>,
            args: *mut &::std::collections::HashMap<
                String,
                ::std::collections::HashMap<String, rustic_extension_api::ExtensionConfigValue>,
            >,
        ) {
            println!("setting up plugin {}", env!("CARGO_PKG_NAME"));
            use rustic_extension_api::*;

            let config = unsafe { Box::from_raw(args) };
            let metadata = <$extension>::metadata();
            let config = config.get(&metadata.id).cloned().unwrap_or_default();
            let extension = <$extension>::new(config);
            insert_instance(plugin, Box::new(extension));
        }
    };
}
