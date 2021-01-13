use std::{convert::TryInto, fmt::Write};

use minecraft_varint::{VarIntRead, VarIntWrite};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::CursorType;

pub trait ReadMCString {
    fn read_mc_string(&mut self) -> Option<String>;
}
pub trait WriteMCString {
    fn write_mc_string< STR: ToString>(&mut self, s: STR) -> Option<()>;
}
impl<C: VarIntRead + std::io::Read> ReadMCString for C {
    fn read_mc_string(&mut self) -> Option<String> {
        let len = self.read_var_i32().ok()?;
        let mut buffer = vec![];
        for _ in 0..len {
            buffer.push(self.read_u8().ok()?);
        }
        Some(unsafe { String::from_utf8_unchecked(buffer) })
    }
}
impl<C: VarIntWrite + std::io::Write> WriteMCString for C {
    fn write_mc_string<STR: ToString>(&mut self, s: STR) -> Option<()>{
        self.write_var_i32(s.to_string().len().try_into().unwrap());
        for c in s.to_string().chars() {
            self.write_u8(c as u8).ok()?;
        }
        Some(())
    }
}
pub trait Packet {
    fn read(s: &mut CursorType) -> Option<Self>
    where
        Self: Sized;
    fn write<W: std::io::Write>(&self, s: &mut W) -> Option<()>;
}
