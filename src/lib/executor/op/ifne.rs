use crate::executor::{stack::Word, Executor};
use anyhow::{bail, Context, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    let operands = executor.stack.pop_operands(1)?;
    let value = operands.first().context("local variables are empty")?;
    if !matches!(value, Word::Int(..)) {
        bail!("value must be Int, is {value}");
    }

    dbg!(value);
    bail!("ifne");
}
