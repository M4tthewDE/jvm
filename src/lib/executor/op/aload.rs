use crate::executor::{stack::Word, Executor};
use anyhow::{Context, Result};

pub fn aload_0(executor: &mut Executor) -> Result<()> {
    let local_variables = executor.stack.local_variables()?;
    let reference = local_variables
        .first()
        .context("local variables are empty")?;
    assert!(matches!(reference, Word::Reference { .. }));
    executor.stack.push_operand(reference.clone());
    executor.pc(1)?;

    Ok(())
}
