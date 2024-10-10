use std::io::{Cursor, Read};

pub mod attribute;
pub mod class;
pub mod constant_pool;
pub mod method;

fn parse_u8(c: &mut Cursor<&Vec<u8>>) -> u8 {
    let mut tag = [0u8; 1];
    c.read_exact(&mut tag).unwrap();
    tag[0]
}

fn parse_u16(c: &mut Cursor<&Vec<u8>>) -> u16 {
    let mut val = [0u8; 2];
    c.read_exact(&mut val).unwrap();
    u16::from_be_bytes(val)
}

fn parse_u32(c: &mut Cursor<&Vec<u8>>) -> u32 {
    let mut val = [0u8; 4];
    c.read_exact(&mut val).unwrap();
    u32::from_be_bytes(val)
}
