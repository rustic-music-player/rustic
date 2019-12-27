#[macro_use]
extern crate rustic_core;

use rustic_core::extension::*;

fn main() {
    let extension = ExtensionBuilder::new("party-mode", "Party Mode", crate_version!())
        .on_add_to_queue(&|tracks| Ok(tracks))
        .build();
    host_extension(extension);
}
