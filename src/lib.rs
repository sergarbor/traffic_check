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
        0x06 => String::from("TCP"),
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
pub fn bytes_to_ethere_type(address_bytes: &[u8]) -> String {
    match address_bytes {
        [0x08, 0x00] => String::from("IPv4"),
        [0x08, 0x06] => String::from("ARP"),
        [0x86, 0xDD] => String::from("IPv6"),
        [0x81, 0x00] => String::from("VLAN Tagged Frame"),
        [0x88, 0x8E] => String::from("IEEE 802.1X Authentication"),
        [0x88, 0xCC] => String::from("LLDP"),
        _ => String::from("Unknown EtherType"),
    }
}

pub fn bytes_to_ip_address(address_bytes: &[u8; 4]) -> String {
    format!(
        "{}.{}.{}.{}",
        address_bytes[0], address_bytes[1], address_bytes[2], address_bytes[3]
    )
}
