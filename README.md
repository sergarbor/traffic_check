
# traffic_check

This is just small a project to practice and learn Rust.

The goal of the project is to have basic traffic analyzing program using the [pcap](https://docs.rs/pcap/latest/pcap/) library to capture the packets.

## Basic Analisys

The fisrt features to be implemented are:
- Get Ethernet frame info
- Get IPV4/IPV6 frame info
- Get UDP/TCP frame info

Next features to develop:
- Create extend the protocols to be parsed
- Start some kind of basic analysis
  - identify apps
  - traffic load per address/app
  - amount of packages or load per app/address
