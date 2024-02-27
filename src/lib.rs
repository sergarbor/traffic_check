use chrono::prelude::*;
use std::convert::TryInto;

pub fn slice_to_num(buff: &[u8]) -> u32 {
    u32::from_ne_bytes(buff.try_into().unwrap())
}

pub fn get_readable_time(time_stamp: String) -> String {
    // Convert the timestamp string into an i64
    let timestamp = time_stamp.parse::<i64>().unwrap();
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    // Print the newly formatted date and time
    newdate.to_string()
}

pub fn bytes_to_protocol(byte: u8) -> String {
    match byte {
        0x01 => String::from("ICMP"),
        0x02 => String::from("IGMP"),
        0x06 => String::from("TCP"),
        0x29 => String::from("IPv6"),
        0x59 => String::from("OSPF"),
        0x11 => String::from("UDP"),
        _ => String::from("UNKNOWN"),
    }
}

/// Transforms an array of 6 bytes into a MAC address
pub fn bytes_to_mac_address(address_bytes: &[u8]) -> String {
    let address: String = format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        address_bytes[0],
        address_bytes[1],
        address_bytes[2],
        address_bytes[3],
        address_bytes[4],
        address_bytes[5]
    );
    address
}

/// Transforms an array of 2 bytes into a ethernet type
pub fn bytes_to_ethere_type(type_bytes: &[u8]) -> String {
    match type_bytes {
        [0x08, 0x00] => String::from("IPv4"),
        [0x08, 0x06] => String::from("ARP"),
        [0x86, 0xDD] => String::from("IPv6"),
        [0x81, 0x00] => String::from("VLAN Tagged Frame"),
        [0x88, 0x64] => String::from("PPPoE"),
        [0x88, 0x8E] => String::from("IEEE 802.1X Authentication"),
        [0x88, 0xCC] => String::from("LLDP"),
        _ => String::from("Unknown EtherType"),
    }
}

/// Transforms an array of 2 bytes into a arp hardwate type (HTYPE)
pub fn bytes_to_arp_hardware_type(hardware_bytes: &[u8]) -> String {
    match hardware_bytes {
        [0x00, 0x01] => String::from("Ethernet (MAC) Address"),
        [0x00, 0x06] => String::from("IEEE 802 Networks"),
        [0x00, 0x0F] => String::from("Serial Line"),
        [0x00, 0x16] => String::from("Frame Relay"),
        [0x00, 0x19] => String::from("ATM"),
        [0x00, 0x1C] => String::from("HDLC"),
        [0x00, 0x24] => String::from("IEEE 1394 (FireWire)"),
        [0x00, 0x27] => String::from("InfiniBand"),
        [0x00, 0x3F] => String::from("Bluetooth"),
        [0x00, 0x42] => String::from("VLANs"),
        [0x00, 0x43] => String::from("VPLS"),
        [0x00, 0x81] => String::from("Zigbee"),
        _ => String::from("Unknown HTYPE"),
    }
}

/// Transforms an array of 2 bytes into a arp protocol type (PTYPE)
pub fn bytes_to_arp_protocol_type(protocol_bytes: &[u8]) -> String {
    match protocol_bytes {
        [0x08, 0x00] => String::from("IPv4"),
        [0x08, 0x06] => String::from("ARP"),
        [0x86, 0xDD] => String::from("IPv6"),
        [0x80, 0x35] => String::from("RARP"),
        [0x80, 0x9B] => String::from("Appletalk"),
        [0x80, 0x9D] => String::from("Appletalk ARP"),
        _ => String::from("Unknown PTYPE"),
    }
}

/// Transforms an array of 2 bytes into a arp protocol type (PTYPE)
pub fn bytes_to_arp_operation(operation_bytes: &[u8]) -> String {
    match operation_bytes {
        [0x00, 0x01] => String::from("ARP Request"),
        [0x00, 0x02] => String::from("ARP Reply"),
        [0x00, 0x03] => String::from("RARP Request"),
        [0x00, 0x04] => String::from("RARP Reply"),
        [0x00, 0x08] => String::from("InARP Request"),
        [0x00, 0x09] => String::from("InARP Reply"),
        _ => String::from("Unknown ARP Operation"),
    }
}

pub fn bytes_to_ip_address(address_bytes: &[u8; 4]) -> String {
    format!(
        "{}.{}.{}.{}",
        address_bytes[0], address_bytes[1], address_bytes[2], address_bytes[3]
    )
}

pub fn is_allowed_protocol(protocols: Vec<String>, type_bytes: &[u8]) -> bool {
    protocols.contains(&bytes_to_ethere_type(&type_bytes)) || protocols.contains(&"ALL".to_string())
}
