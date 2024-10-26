use crate::parser::parse_u16;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    pub fn new(c: &mut std::io::Cursor<&Vec<u8>>) -> Result<Self> {
        let bootstrap_method_ref = parse_u16(c)?;
        let num_bootstrap_arguments = parse_u16(c)? as usize;

        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments);
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(parse_u16(c)?);
        }

        Ok(Self {
            bootstrap_method_ref,
            bootstrap_arguments,
        })
    }
}
