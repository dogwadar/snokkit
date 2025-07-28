use crate::utils::*;
use colored::*;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;

pub fn start_capture(interface_name: String, filter: Option<String>) {
    let ethertype_map = build_ethertype_map();
    let filter_map = build_filter_map();

    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .expect("Failed to find the specified interface");

    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error creating datalink channel: {}", e),
    };

    println!("\nCapturing packets on interface: {interface_name}\n\n");

    let expected_ethertype = filter
        .as_ref()
        .and_then(|f| filter_map.get(&f.to_lowercase()))
        .copied();

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                // Skip if filter set and packet doesn't match
                if let Some(expected) = expected_ethertype {
                    if packet.get_ethertype() != expected {
                        continue;
                    }
                }

                let timestamp = get_timestamp();
                let source = format!("{}", packet.get_source()).cyan();
                let destination = format!("{}", packet.get_destination()).magenta();

                let protocol = ethertype_map
                    .get(&packet.get_ethertype())
                    .cloned()
                    .unwrap_or_else(|| format!("Other (0x{:04x})", packet.get_ethertype().0).red());

                println!(
                    "{} {} → {} [{}]",
                    timestamp.dimmed(),
                    source,
                    destination,
                    protocol
                );
            }
            Err(e) => {
                eprintln!("Error while reading: {}", e.to_string().red());
            }
        }
    }
}
