use crate::parser::parse_u16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumberTableEntry {
    pub fn new(c: &mut std::io::Cursor<&Vec<u8>>) -> LineNumberTableEntry {
        LineNumberTableEntry {
            start_pc: parse_u16(c),
            line_number: parse_u16(c),
        }
    }
}
