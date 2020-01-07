use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crossbeam_channel::Sender;

struct Emitter {
    sender: Sender<()>,
    emitted: bool,
}

impl Emitter {
    fn emit(&mut self) {
        if self.emitted {
            return;
        }
        self.emitted = true;
        self.sender.send(()).unwrap();
    }
}

pub(crate) struct RodioFile(File, Emitter);

impl RodioFile {
    pub fn open<P: AsRef<Path>>(path: P, sender: Sender<()>) -> io::Result<RodioFile> {
        let file = File::open(path)?;
        Ok(RodioFile(
            file,
            Emitter {
                sender,
                emitted: false,
            },
        ))
    }
}

impl Read for RodioFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let count = self.0.read(buf)?;
        if count == 0 {
            self.1.emit();
        }
        Ok(count)
    }
}

impl Seek for RodioFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
        self.0.seek(pos)
    }
}
