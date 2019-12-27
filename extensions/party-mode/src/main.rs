use rustic_core::extension::*;
use rustic_core::Track;
use std::error::Error;

struct PartyMode;

impl Extension for PartyMode {
    fn id(&self) -> String {
        String::from("party-mode")
    }

    fn name(&self) -> String {
        String::from("Party Mode")
    }

    fn on_add_to_queue(&mut self, tracks: Vec<Track>) -> Result<Vec<Track>, Box<dyn Error>> {
        Ok(tracks)
    }
}

fn main() {
    let extension = PartyMode;
    host_extension(extension);
}
