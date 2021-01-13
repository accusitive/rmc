use std::{convert::TryInto, io::Cursor};

use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use minecraft_varint::{VarIntRead, VarIntWrite};

use crate::network::{Packet, ReadMCString, WriteMCString};
use crate::CursorType;
#[derive(Debug)]
// pub struct HandshakeC2S(i32, String, u16, i32);
pub struct HandshakeC2S {
    pub protocol_version: i32,
    pub address: String,
    pub port: u16,
    pub next_state: i32,
}
pub struct LoginStartC2S {
    pub name: String,
}
pub struct LoginSuccessS2C {
    pub uuid: i128,
    pub username: String
}
impl Packet for HandshakeC2S {
    fn read(s: &mut CursorType) -> Option<Self> {
        let protocol_version = s.read_var_i32().ok()?;
        let address = s.read_mc_string().unwrap();
        let port = s.read_u16::<BigEndian>().ok()?;
        let next_state = s.read_var_i32().ok()?;
        Some(Self {
            protocol_version,
            address,
            port,
            next_state,
        })
    }

    fn write<W: std::io::Write>(&self, s: &mut W) -> Option<()> {
        None
    }
}

impl Packet for LoginStartC2S {
    fn read(s: &mut CursorType) -> Option<Self>
    where
        Self: Sized,
    {
        let name = s.read_mc_string()?;
        Some(Self { name })
    }

    fn write<W: std::io::Write>(&self, s: &mut W) -> Option<()> {
        None
    }
}

impl Packet for LoginSuccessS2C {
    fn read(s: &mut CursorType) -> Option<Self>
    where
        Self: Sized {
        None
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> Option<()> {
        let temp = Vec::with_capacity(2048);
        let mut cursor = Cursor::new(temp);
        cursor.write_i128::<BigEndian>(self.uuid).ok()?;
        cursor.write_mc_string(self.username.clone())?;
        let inner = cursor.into_inner();
        w.write_var_i32(0x02).ok()?;
        w.write_var_i32(inner.len().try_into().unwrap()).ok()?;
        w.write(&inner).ok()?;
        Some(())
    }
}