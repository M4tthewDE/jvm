use crate::{executor::Executor, parser::constant_pool::Index};
use anyhow::{bail, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let indexbyte1 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let indexbyte2 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;

    let field_ref_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.resolve_field(&field_ref_index)?;
    bail!("execute_getstatic")
}
