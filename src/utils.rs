use chrono::Local;
use colored::*;
use pnet::packet::ethernet::{EtherType, EtherTypes};
use std::collections::HashMap;

pub fn build_ethertype_map() -> HashMap<EtherType, ColoredString> {
    let mut map = HashMap::new();

    map.insert(EtherTypes::Ipv4, "IPv4".green());
    map.insert(EtherTypes::Ipv6, "IPv6".blue());
    map.insert(EtherTypes::Arp, "ARP".yellow());
    map.insert(EtherTypes::Rarp, "RARP".bright_yellow());
    map.insert(EtherTypes::PppoeDiscovery, "PPPoE-Discovery".bright_cyan());
    map.insert(EtherTypes::PppoeSession, "PPPoE-Session".cyan());
    map.insert(EtherTypes::Mpls, "MPLS".bright_blue());
    map.insert(EtherTypes::MplsMcast, "MPLS-MCAST".blue());
    map.insert(EtherTypes::WakeOnLan, "Wake-on-LAN".bright_green());
    map.insert(EtherTypes::Vlan, "802.1Q VLAN".bright_white());

    map
}

pub fn build_filter_map() -> HashMap<String, EtherType> {
    let mut map = HashMap::new();

    map.insert("ipv4".to_string(), EtherTypes::Ipv4);
    map.insert("ipv6".to_string(), EtherTypes::Ipv6);
    map.insert("arp".to_string(), EtherTypes::Arp);
    map.insert("rarp".to_string(), EtherTypes::Rarp);
    map.insert("pppoe-discovery".to_string(), EtherTypes::PppoeDiscovery);
    map.insert("pppoe-session".to_string(), EtherTypes::PppoeSession);
    map.insert("mpls".to_string(), EtherTypes::Mpls);
    map.insert("mpls-mcast".to_string(), EtherTypes::MplsMcast);
    map.insert("wakeonlan".to_string(), EtherTypes::WakeOnLan);
    map.insert("vlan".to_string(), EtherTypes::Vlan);

    map
}

pub fn get_timestamp() -> String {
    Local::now().format("%H:%M:%S%.3f").to_string()
}
