use std::io::Cursor;

use crate::parser::{constant_pool::Index, parse_u16};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name_index: Index,
    descriptor_index: Index,
    index: usize,
}

impl LocalVariable {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self {
            start_pc: parse_u16(c),
            length: parse_u16(c),
            name_index: Index::new(parse_u16(c)),
            descriptor_index: Index::new(parse_u16(c)),
            index: parse_u16(c) as usize,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: Index,
    signature_index: Index,
    index: usize,
}

impl LocalVariableType {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self {
            start_pc: parse_u16(c),
            length: parse_u16(c),
            name_index: Index::new(parse_u16(c)),
            signature_index: Index::new(parse_u16(c)),
            index: parse_u16(c) as usize,
        }
    }
}
