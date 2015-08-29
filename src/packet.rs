use std::cmp::Ordering;
use bincode;

#[derive(Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Flag {
    SYN,
    ACK,
    PSH,
    RST,
    FIN,
    ERR,
}

#[derive(Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Packet {
    pub flag: Flag,
    pub seq_num: u32,
    pub msg: Vec<u8>, // Remember kids, you can't serialize scalar arrays!
}

impl Packet {
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::encode(&self, bincode::SizeLimit::Bounded(1024))
                 .ok()
                 .unwrap_or(Vec::new())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        other.seq_num.cmp(&self.seq_num)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn to_packet(bytes: &[u8]) -> Packet {
    bincode::decode(bytes).unwrap_or(Packet{flag: Flag::ERR,
                                            seq_num: 0,
                                            msg: Vec::new()})
}
