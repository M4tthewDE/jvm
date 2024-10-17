use std::io::Cursor;

use crate::parser::parse_u16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariable {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self {
            start_pc: parse_u16(c),
            length: parse_u16(c),
            name_index: parse_u16(c),
            descriptor_index: parse_u16(c),
            index: parse_u16(c),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl LocalVariableType {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self {
            start_pc: parse_u16(c),
            length: parse_u16(c),
            name_index: parse_u16(c),
            signature_index: parse_u16(c),
            index: parse_u16(c),
        }
    }
}
