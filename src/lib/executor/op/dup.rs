use crate::executor::Executor;
use anyhow::Result;

pub fn perform(executor: &mut Executor) -> Result<()> {
    let operands = executor.stack.pop_operands(1)?;
    executor.stack.push_operand(operands[0].clone());
    executor.stack.push_operand(operands[0].clone());
    executor.pc(1)?;

    Ok(())
}
