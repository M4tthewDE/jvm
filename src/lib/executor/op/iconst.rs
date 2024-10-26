use crate::executor::{stack::Word, Executor};
use anyhow::Result;

pub fn iconst_0(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    executor.stack.push_operand(Word::Int(0));

    Ok(())
}
