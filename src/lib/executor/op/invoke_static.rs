use crate::{executor::Executor, parser::constant_pool::Index};
use anyhow::Result;

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let indexbyte1 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let indexbyte2 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);

    let (class_identifier, name_and_type) = executor.stack.lookup_method(&method_index)?;
    executor.invoke_static(class_identifier, name_and_type)
}
