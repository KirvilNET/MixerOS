


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MsgType {
    Handshake = 0x01,
    HeartBeat = 0x02,
    Channel = 0x3,
    Bus = 0x4,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Header {
    version: u8,
    msg_type: u16,
    
}