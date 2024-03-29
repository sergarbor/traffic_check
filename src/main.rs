use clap::Parser;
use pcap::Device;
use traffic_check::{bytes_to_ethere_type, get_readable_time};

mod models {
    // Import the structs from each model file
    pub mod arp_frame;
    pub mod cli;
    pub mod ethernet_frame;
    pub mod ipv4_frame;
}
fn capture_packets(n_packets: u8) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    let mut cap_counter = 0;

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }

        let packet_data: &[u8] = packet.data;
        let packet_len: u32 = packet.header.len;

        // Get the ethernet layer info
        let ether_bytes: &[u8] = &packet_data;
        let ether_header = models::ethernet_frame::EthernetFrame::new(ether_bytes);

        //let protocol: String = get_protocol_from_data(packet_data);
        let capture_time = get_readable_time(packet.header.ts.tv_sec.to_string());

        let ether_info = ether_header.to_string();

        let mut ipv4_info = String::from("");
        if bytes_to_ethere_type(&ether_header.ether_type) == "IPv4" {
            // get the IPV4 layer info
            let ipv4_bytes: &[u8] = &packet_data[14..];
            let ipv4_header = models::ipv4_frame::IPV4Frame::new(ipv4_bytes);
            ipv4_info = ipv4_header.to_string();
        }

        let mut arp_info = String::from("");
        if bytes_to_ethere_type(&ether_header.ether_type) == "ARP" {
            // get the ARP layer info
            let arp_bytes: &[u8] = &packet_data[14..];
            let arp_header = models::arp_frame::ARPFrame::new(arp_bytes);
            arp_info = arp_header.to_string();
        }

        println!(
            "----------------------------------------
            - {} - Len: {}
            \t{}
            \t{}
            \t{}",
            capture_time,
            packet_len,
            //protocol,
            ether_info,
            ipv4_info,
            arp_info,
        );

        cap_counter += 1;
    }
}

fn main() {
    let args = models::cli::Cli::parse();

    println!("Command: {} Value {}", args.command, args.value);

    // If the command is not check we skip
    if args.command != "check" {
        return;
    }

    println!("THIS IS THE EXPECTED COMMAND!");

    // Start capturing packets!!
    capture_packets(args.value);
}
