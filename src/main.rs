use clap::Parser;
use pcap::{Capture, Device};
use traffic_check::{bytes_to_ethere_type, get_readable_time, is_allowed_protocol};

mod models {
    // Import the structs from each model file
    pub mod arp_frame;
    pub mod cli;
    pub mod ethernet_frame;
    pub mod ipv4_frame;
}
fn capture_packets(n_packets: u8, protocols: Vec<String>) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    let mut cap_counter = 0;
    let string_layer_separator = "\n";

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }
        let mut output_str = String::from("");

        let packet_data: &[u8] = packet.data;
        let packet_len: u32 = packet.header.len;

        // Get the ethernet layer info
        let ether_bytes: &[u8] = &packet_data;
        let ether_header = models::ethernet_frame::EthernetFrame::new(ether_bytes);

        //let protocol: String = get_protocol_from_data(packet_data);
        let capture_time = get_readable_time(packet.header.ts.tv_sec.to_string());

        let ether_info = ether_header.to_string();
        output_str += &string_layer_separator.to_string();
        output_str += &ether_info;

        // check that the protocol is in the list
        if !is_allowed_protocol(protocols.clone(), &ether_header.ether_type) {
            continue;
        }

        let mut ipv4_info = String::from("");
        if bytes_to_ethere_type(&ether_header.ether_type) == "IPv4" {
            // get the IPV4 layer info
            let ipv4_bytes: &[u8] = &packet_data[14..];
            let ipv4_header = models::ipv4_frame::IPV4Frame::new(ipv4_bytes);
            ipv4_info = ipv4_header.to_string();
            output_str += &string_layer_separator.to_string();
            output_str += &ipv4_info;
        }

        let mut arp_info = String::from("");
        if bytes_to_ethere_type(&ether_header.ether_type) == "ARP" {
            // get the ARP layer info
            let arp_bytes: &[u8] = &packet_data[14..];
            let arp_header = models::arp_frame::ARPFrame::new(arp_bytes);
            arp_info = arp_header.to_string();
            output_str += &string_layer_separator.to_string();
            output_str += &arp_info;
        }

        println!(
            "----------------------------------------
            - {} - Len: {}
            \t{}",
            capture_time, packet_len, output_str,
        );

        cap_counter += 1;
    }
}

fn arp_scann() {
    let mut cap = Capture::from_device(Device::lookup().unwrap().unwrap())
        .unwrap()
        .promisc(true)
        .open()
        .unwrap();
    let mut cap_counter = 0;
    let mut protocols: Vec<String> = Vec::new();
    let n_packets = 30;
    protocols.push(String::from("ARP"));

    while let Ok(packet) = cap.next_packet() {
        if cap_counter >= n_packets {
            break;
        }

        let packet_data: &[u8] = packet.data;
        // Get the ethernet layer info
        let ether_bytes: &[u8] = &packet_data;
        let ether_header = models::ethernet_frame::EthernetFrame::new(ether_bytes);

        if !is_allowed_protocol(protocols.clone(), &ether_header.ether_type) {
            continue;
        }

        // get the ARP layer info
        let arp_bytes: &[u8] = &packet_data[14..];
        let arp_header = models::arp_frame::ARPFrame::new(arp_bytes);
        let mut arp_info = String::from("");
        arp_info = arp_header.to_scann_string();
        println!(" {}", arp_info);

        cap_counter += 1
    }
}

fn main() {
    let args = models::cli::Cli::parse();
    let mut protocols: Vec<String> = Vec::new();
    // We use all as default

    match args {
        models::cli::Cli::Save(save_args) => {
            if let Some(filename) = save_args.filename {
                println!("Save command with filename: {}", filename);
            } else {
                println!("Save command without filename");
            }
        }
        models::cli::Cli::Show => {
            println!("Show command");
        }
        models::cli::Cli::Output(output_args) => {
            println!("Output command with filename: {}", output_args.filename);
        }
        models::cli::Cli::Protocols(protocols_args) => {
            protocols = protocols_args.protocols;
            if protocols.is_empty() {
                protocols.push(String::from("ALL"));
            }
            println!("Protocols command with protocols: {:?}", protocols);
        }
    }

    println!("THIS IS THE EXPECTED COMMAND!");

    // Start capturing packets!!
    //capture_packets(100, protocols);
    arp_scann();
}
