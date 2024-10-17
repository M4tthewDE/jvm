use std::io::Cursor;

use crate::parser::parse_u16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Exception {
    pub(crate) fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self {
            start_pc: parse_u16(c),
            end_pc: parse_u16(c),
            handler_pc: parse_u16(c),
            catch_type: parse_u16(c),
        }
    }
}
