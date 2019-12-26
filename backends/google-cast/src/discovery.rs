use std::net::IpAddr;

use log::{debug, trace};
use mdns::{self, Record, RecordKind};
use std::sync::{Condvar, Mutex, Arc};

#[derive(Debug)]
pub(crate) enum DiscoverMessage {
    AddBackend(Target)
}

#[derive(Debug, Clone)]
pub(crate) struct Target {
    pub name: String,
    pub addr: IpAddr,
}

const SERVICE_NAME: &'static str = "_googlecast._tcp.local";

pub(crate) fn discover(sender: crossbeam_channel::Sender<DiscoverMessage>,
                       running: Arc<(Mutex<bool>, Condvar)>) {
    debug!("discovering...");
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        trace!("done");
        let response = response.unwrap();

        let target = response.records()
            .filter_map(self::to_target)
            .next();

        if let Some(target) = target {
            debug!("found cast device at {}", target.addr);
            sender.send(DiscoverMessage::AddBackend(target))
        } else {
            trace!("cast device does not advertise address");
        }
    }
}

fn to_target(record: &Record) -> Option<Target> {
    trace!("{:?}", record);
    let addr = match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    };
    addr.map(|addr| Target {
        name: record.name.clone(),
        addr,
    })
}
