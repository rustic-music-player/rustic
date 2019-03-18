pub enum DiscoverMessage {
    AddBackend(GoogleCastBackend),
    RemoveBackend(GoogleCastBackend)
}

const SERVICE_NAME: &'static str = "_googlecast._tcp.local";

use mdns::{Record, RecordKind, self};
use std::net::IpAddr;

fn discover() {
    println!("discovering...");
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        println!("done");
        let response = response.unwrap();

        let addr = response.records()
            .filter_map(self::to_ip_addr)
            .next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    println!("{:?}", record);
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
