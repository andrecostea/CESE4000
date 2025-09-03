struct UdpHeader{
    source: u128,
    destination: u128,
    udp_length: u32,
    next: u8,
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
}

impl UdpHeader{
    pub const ZEROS: u8 = 0;   
}

fn main() {

}
