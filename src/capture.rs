use crate::utils::*;
use colored::*;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

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

    let expected_ethertype = filter
        .as_ref()
        .and_then(|f| filter_map.get(&f.to_lowercase()))
        .copied();

    println!("\nCapturing packets on interface: {interface_name}\n\n");

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

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

                match packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        if let Some(ipv4) = Ipv4Packet::new(packet.payload()) {
                            handle_transport_protocol(
                                ipv4.get_source().to_string(),
                                ipv4.get_destination().to_string(),
                                ipv4.get_next_level_protocol(),
                                ipv4.payload(),
                            );
                        }
                    }
                    EtherTypes::Ipv6 => {
                        if let Some(ipv6) = Ipv6Packet::new(packet.payload()) {
                            handle_transport_protocol(
                                ipv6.get_source().to_string(),
                                ipv6.get_destination().to_string(),
                                ipv6.get_next_header(),
                                ipv6.payload(),
                            );
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Error while reading: {}", e.to_string().red());
            }
        }
    }
}

fn handle_transport_protocol(
    src_ip: String,
    dst_ip: String,
    protocol: IpNextHeaderProtocol,
    payload: &[u8],
) {
    match protocol {
        IpNextHeaderProtocols::Tcp => handle_tcp_packet(src_ip, dst_ip, payload),
        IpNextHeaderProtocols::Udp => handle_udp_packet(src_ip, dst_ip, payload),
        IpNextHeaderProtocols::Icmp | IpNextHeaderProtocols::Icmpv6 => {
            println!("ICMP packet: {} → {}", src_ip, dst_ip);
        }
        other => {
            println!(
                "Unhandled protocol {:?} from {} to {}",
                other, src_ip, dst_ip
            );
        }
    }
}

fn handle_tcp_packet(src_ip: String, dst_ip: String, payload: &[u8]) {
    if let Some(tcp) = TcpPacket::new(payload) {
        let src_port = tcp.get_source();
        let dst_port = tcp.get_destination();
        let seq = tcp.get_sequence();
        let ack = tcp.get_acknowledgement();
        let flags = tcp.get_flags();

        println!(
            "TCP {}:{} → {}:{} | SEQ={} ACK={} FLAGS=0x{:02x}",
            src_ip, src_port, dst_ip, dst_port, seq, ack, flags
        );
    } else {
        eprintln!("Malformed TCP packet from {} to {}", src_ip, dst_ip);
    }
}

fn handle_udp_packet(src_ip: String, dst_ip: String, payload: &[u8]) {
    if let Some(udp) = UdpPacket::new(payload) {
        let src_port = udp.get_source();
        let dst_port = udp.get_destination();
        let length = udp.get_length();

        println!(
            "UDP {}:{} → {}:{} | Length={}",
            src_ip, src_port, dst_ip, dst_port, length
        );
    } else {
        eprintln!("Malformed UDP packet from {} to {}", src_ip, dst_ip);
    }
}
