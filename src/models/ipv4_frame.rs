use traffic_check::{bytes_to_ip_address, bytes_to_protocol};

/* -------------------------- IPV4 PARSER -------------------------- */
pub struct IPV4Frame {
    /* Basic fields */
    pub dest_address: [u8; 4],
    pub src_address: [u8; 4],
    pub protocol: u8,
    /* Extended fields */
    // pub version:
}

impl IPV4Frame {
    pub fn new(ipv4_bytes: &[u8]) -> Self {
        let mut dest_address = [0; 4];
        dest_address.copy_from_slice(&ipv4_bytes[16..20]);

        let mut src_address = [0; 4];
        src_address.copy_from_slice(&ipv4_bytes[12..16]);

        let protocol: u8;
        if let Some(&byte) = ipv4_bytes.get(9) {
            protocol = byte;
        } else {
            protocol = 0x00;
        }

        Self {
            dest_address,
            src_address,
            protocol,
        }
    }

    pub fn to_string(&self) -> String {
        let ret: String = format!(
            "Dest. address: {}, Source address: {}, IP type: {}",
            bytes_to_ip_address(&self.dest_address),
            bytes_to_ip_address(&self.src_address),
            bytes_to_protocol(self.protocol),
        );
        ret
    }
}
