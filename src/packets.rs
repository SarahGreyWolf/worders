use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::Cursor;

pub trait PacketFrom {
    fn decode(input: &mut Cursor<&[u8]>) -> Self;
}

pub trait PacketTo {
    fn length(self) -> usize;
    fn encode<T: Write>(self, writer: &mut T);
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
    fn encode<T: Write>(self, writer: &mut T) {
        writer.write_u8(self).unwrap();
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
    fn encode<T: Write>(self, writer: &mut T) {
        writer.write_u16::<BigEndian>(self).unwrap();
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
    fn encode<T: Write>(self, writer: &mut T) {
        writer.write_u32::<BigEndian>(self).unwrap();
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
    fn encode<T: Write>(self, writer: &mut T) {
        writer.write_u64::<BigEndian>(self).unwrap();
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
    fn encode<T: Write>(self, writer: &mut T) {
        writer.write_u8(self as u8).unwrap();
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

    fn encode<T: Write>(self, writer: &mut T) {
        let length = self.clone().length() as u16;
        length.encode(writer);
        writer.write(self.as_bytes()).unwrap();
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
    fn encode<T: Write>(self, writer: &mut T) {
        let length = self.clone().length() as u16;
        length.encode(writer);
        writer
            .write(
                self.as_slice()
                    .iter()
                    .map(|c| *c as u8)
                    .collect::<Vec<u8>>()
                    .as_slice(),
            )
            .unwrap();
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
            fn encode<T: Write>(self, writer: &mut T) {
                $(self.$v.encode(writer);)*
                writer.flush().unwrap();
            }
        }
    }
}

dec_packet!(PlayerState{id: u16, tiles: Vec<char>, score: u8});
dec_packet!(OtherPlayerState {
    id: u16,
    username: String,
    score: u8
});
dec_packet!(GameState{id: u8, placed: Vec<char>});
dec_packet!(Place {
    id: u8,
    tile: char,
    x: u32,
    y: u32
});
