use traffic_check::{bytes_to_ethere_type, bytes_to_mac_address};

/* -------------------------- ETHERNET PARSER -------------------------- */
pub struct EthernetFrame {
    pub dest_address: [u8; 6],
    pub src_address: [u8; 6],
    pub ether_type: [u8; 2],
}

impl EthernetFrame {
    pub fn new(ether_bytes: &[u8]) -> Self {
        let mut dest_address = [0; 6];
        dest_address.copy_from_slice(&ether_bytes[0..6]);

        let mut src_address = [0; 6];
        src_address.copy_from_slice(&ether_bytes[6..12]);

        let mut ether_type = [0; 2];
        ether_type.copy_from_slice(&ether_bytes[12..14]);

        Self {
            dest_address,
            src_address,
            ether_type,
        }
    }

    pub fn to_string(&self) -> String {
        let ret: String = format!(
            "Dest. address: {}, Source address: {}, Eth. type: {}",
            bytes_to_mac_address(&self.dest_address),
            bytes_to_mac_address(&self.src_address),
            bytes_to_ethere_type(&self.ether_type),
        );
        return ret;
    }
}
