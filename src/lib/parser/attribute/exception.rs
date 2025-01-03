use crate::parser::parse_u16;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Exception {
    pub fn new(c: &mut std::io::Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Self {
            start_pc: parse_u16(c)?,
            end_pc: parse_u16(c)?,
            handler_pc: parse_u16(c)?,
            catch_type: parse_u16(c)?,
        })
    }
}
