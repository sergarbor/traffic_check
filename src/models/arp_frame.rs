use traffic_check::{bytes_to_arp_hardware_type, bytes_to_arp_protocol_type, bytes_to_mac_address};

pub struct ARPFrame {
    pub hardware_type: [u8; 2],
    pub protocol_type: [u8; 2],
    pub hardware_address_len: u8,
    pub protocol_address_len: u8,
    pub operation: [u8; 2],
    pub sender_hardware_address: [u8; 6],
    pub sender_protocol_address: [u8; 4],
    pub target_hardware_address: [u8; 6],
    pub target_protocol_address: [u8; 4],
}

impl ARPFrame {
    pub fn new(arp_bytes: &[u8]) -> Self {
        let mut hardware_type = [0; 2];
        hardware_type.copy_from_slice(&arp_bytes[0..2]);

        let mut protocol_type = [0; 2];
        protocol_type.copy_from_slice(&arp_bytes[2..4]);

        let hardware_address_len: u8;
        if let Some(&byte) = arp_bytes.get(4) {
            hardware_address_len = byte;
        } else {
            hardware_address_len = 0x00;
        }

        let protocol_address_len: u8;
        if let Some(&byte) = arp_bytes.get(5) {
            protocol_address_len = byte;
        } else {
            protocol_address_len = 0x00;
        }

        let mut operation = [0; 2];
        operation.copy_from_slice(&arp_bytes[6..8]);

        let mut sender_hardware_address = [0; 6];
        sender_hardware_address.copy_from_slice(&arp_bytes[8..14]);

        let mut sender_protocol_address = [0; 4];
        sender_protocol_address.copy_from_slice(&arp_bytes[14..18]);

        let mut target_hardware_address = [0; 6];
        target_hardware_address.copy_from_slice(&arp_bytes[18..24]);

        let mut target_protocol_address = [0; 4];
        target_protocol_address.copy_from_slice(&arp_bytes[24..28]);

        Self {
            hardware_type,
            protocol_type,
            hardware_address_len,
            protocol_address_len,
            operation,
            sender_hardware_address,
            sender_protocol_address,
            target_hardware_address,
            target_protocol_address,
        }
    }

    pub fn to_string(&self) -> String {
        let ret: String = format!(
            "HTYPE: {}, PTYPE: {}, Sender Addr: {}, Target Addr: {}",
            bytes_to_arp_hardware_type(&self.hardware_type),
            bytes_to_arp_protocol_type(&self.protocol_type),
            bytes_to_mac_address(&self.sender_hardware_address),
            bytes_to_mac_address(&self.target_hardware_address),
        );
        ret
    }
}
