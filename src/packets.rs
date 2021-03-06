use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Result as IoResult;

pub trait PacketFrom {
    fn decode(input: &mut Cursor<&[u8]>) -> Self;
}

pub trait PacketTo {
    fn length(self) -> usize;
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()>;
}

impl PacketFrom for u8 {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        input.read_u8().unwrap()
    }
}

impl PacketTo for u8 {
    fn length(self) -> usize {
        std::mem::size_of::<u8>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u8(self).unwrap();
        Ok(())
    }
}

impl PacketFrom for u16 {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        input.read_u16::<BigEndian>().unwrap()
    }
}

impl PacketTo for u16 {
    fn length(self) -> usize {
        std::mem::size_of::<u16>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u16::<BigEndian>(self).unwrap();
        Ok(())
    }
}

impl PacketFrom for u32 {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        input.read_u32::<BigEndian>().unwrap()
    }
}

impl PacketTo for u32 {
    fn length(self) -> usize {
        std::mem::size_of::<u32>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u32::<BigEndian>(self).unwrap();
        Ok(())
    }
}

impl PacketFrom for u64 {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        input.read_u64::<BigEndian>().unwrap()
    }
}

impl PacketTo for u64 {
    fn length(self) -> usize {
        std::mem::size_of::<u64>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u64::<BigEndian>(self).unwrap();
        Ok(())
    }
}

impl PacketFrom for char {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        input.read_u8().unwrap() as char
    }
}

impl PacketTo for char {
    fn length(self) -> usize {
        // Rust actually stores characters as u16
        // Since unicode characters can be 2 bytes, but we only care about ASCII letters
        std::mem::size_of::<u8>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u8(self as u8).unwrap();
        Ok(())
    }
}

impl PacketFrom for String {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        let length = input.read_u16::<BigEndian>().unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        let bytes_read = input.read_exact(&mut buffer);
        assert!(bytes_read.is_ok());
        String::from_utf8(buffer).unwrap()
    }
}

impl PacketTo for String {
    fn length(self) -> usize {
        std::mem::size_of::<char>() * self.len()
    }

    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        let length = self.clone().length() as u16;
        length.encode(writer)?;
        writer.write(self.as_bytes()).unwrap();
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum AckState {
    Success = 0,
    Confirm = 1,
    Turn = 2,
    Failure = 3,
}

impl From<u8> for AckState {
    fn from(val: u8) -> Self {
        match val {
            0 => AckState::Success,
            1 => AckState::Confirm,
            2 => AckState::Turn,
            _ => AckState::Failure,
        }
    }
}

impl From<AckState> for u8 {
    fn from(val: AckState) -> Self {
        match val {
            AckState::Success => 0,
            AckState::Confirm => 1,
            AckState::Turn => 2,
            AckState::Failure => 3,
        }
    }
}

impl PacketFrom for AckState {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        AckState::from(input.read_u8().unwrap())
    }
}

impl PacketTo for AckState {
    fn length(self) -> usize {
        std::mem::size_of::<u8>()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        writer.write_u8(self.into()).unwrap();
        Ok(())
    }
}

impl PacketFrom for Vec<char> {
    fn decode(input: &mut Cursor<&[u8]>) -> Self {
        let length = input.read_u16::<BigEndian>().unwrap() as usize;
        let mut buffer: Vec<u8> = vec![0u8; length];
        let bytes_read = input.read(&mut buffer);
        assert!(bytes_read.is_ok());
        let content = buffer.iter().map(|b| *b as char).collect::<Vec<char>>();
        content
    }
}

impl PacketTo for Vec<char> {
    fn length(self) -> usize {
        std::mem::size_of::<u8>() * self.len()
    }
    fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
        let length = self.clone().length() as u16;
        length.encode(writer)?;
        writer
            .write(
                self.as_slice()
                    .iter()
                    .map(|c| *c as u8)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )
            .unwrap();
        Ok(())
    }
}

macro_rules! dec_packet {
    ($name:ident{$($v:tt:$t:ty),*}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($v: $t),*
        }

        impl $name {
            pub fn new($($v: $t),*) -> $name {
                Self {
                    $(
                        $v
                    ),*
                }
            }
        }

        impl PacketFrom for $name {
            fn decode(input: &mut Cursor<&[u8]>) -> Self {
                $(
                let $v: $t = <$t>::decode(input);
                )*
                Self {
                    $(
                    $v
                    ),*
                }
            }
        }
        impl PacketTo for $name {
            fn length(self) -> usize {
                $(self.$v.length()+)*0
            }
            fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
                $(self.$v.encode(writer)?;)*
                writer.flush().unwrap();
                Ok(())
            }
        }
    }
}

macro_rules! dec_packets {
    ($($id:literal:$name:ident{$($v:tt:$t:ty),*};)*) => {
        pub enum Packets {
            $(
                $name($name)
            ),*,
            Unknown
        }

        impl PacketFrom for Packets {
            fn decode(cursor: &mut Cursor<&[u8]>) -> Self {
                let packet_id: u8 = u8::decode(cursor);
                match packet_id {
                    $($id => Self::$name($name::decode(cursor)),)*
                    _ => Self::Unknown
                }
            }
        }

        impl PacketTo for Packets {
            fn length(self) -> usize {
                let length = match self {
                    $(Self::$name(packet) => packet.length(),)*
                    Self::Unknown => 0,
                };
                length + std::mem::size_of::<u8>()
            }

            fn encode<T: Write>(self, writer: &mut T) -> IoResult<()> {
                match self {
                    $(Self::$name(packet) => {
                        let id = $id as u8;
                        id.encode(writer)?;
                        packet.encode(writer)?;
                        Ok(())
                    })*
                    Self::Unknown => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown Packet!"))
                }
            }
        }

        $(dec_packet!($name{$($v:$t),*});)*
    }
}

dec_packets!(
    0:Ack {id: u16, state: AckState};
    1:PlayerState {id: u16, player: u8, username: String, tiles: Vec<char>, score: u8};
    2:Place {id: u16, tile: char, x: u32, y: u32};
    3:GameState {id: u16, placed: Vec<char>};
);
