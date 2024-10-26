use crate::{executor::Executor, parser::descriptor::ReturnDescriptor};
use anyhow::{bail, Context, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    let method = executor.stack.current_method()?;

    if method.descriptor.return_descriptor != ReturnDescriptor::Void {
        bail!("only void return supported");
    }

    executor.stack.pop().context("pop failed")?;
    Ok(())
}
