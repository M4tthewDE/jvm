use crate::{executor::Executor, parser::descriptor::ReturnDescriptor};
use anyhow::{Context, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    let method = executor.stack.current_method()?;
    assert_eq!(method.descriptor.return_descriptor, ReturnDescriptor::Void);
    executor.stack.pop().context("pop failed")?;
    Ok(())
}
