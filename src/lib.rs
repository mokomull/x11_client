#[cfg(test)]
mod tests;

extern crate byteorder;

pub struct ClientInit<'a> {
    major: u16,
    minor: u16,
    authorization_protocol_name: Option<&'a [u8]>,
    authorization_protocol_data: Option<&'a [u8]>,
}

impl<'a> ClientInit<'a> {
    pub fn new() -> Self {
        ClientInit {
            major: 11,
            minor: 0,
            authorization_protocol_name: None,
            authorization_protocol_data: None,
        }
    }
}

impl<'a> Into<Vec<u8>> for ClientInit<'a> {
    fn into(self: Self) -> Vec<u8> {
        use std::io::Write;
        use byteorder::{BigEndian, WriteBytesExt};

        let mut bytes = [0 as u8; 2];
        let mut ret = Vec::new();

        ret.write(b"B\x00");
        ret.write_u16::<BigEndian>(self.major);
        ret.write_u16::<BigEndian>(self.minor);
        assert!(self.authorization_protocol_name.is_none());
        assert!(self.authorization_protocol_data.is_none());
        ret.write_u16::<BigEndian>(0);
        ret.write_u16::<BigEndian>(0);
        // the unused data needs to be sent, too.
        ret.write_u16::<BigEndian>(0);
        ret
    }
}
