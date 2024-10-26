use crate::{
    executor::{
        instance::Instance,
        stack::{Reference, Word},
        Executor,
    },
    parser::constant_pool::Index,
};
use anyhow::Result;

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let indexbyte1 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let indexbyte2 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let class_index = Index::new((indexbyte1 << 8) | indexbyte2);

    let identifier = executor.stack.lookup_class(&class_index)?;
    let class = executor.resolve_class(identifier)?;
    let instance = Instance::new(class);
    let reference = Word::Reference(Reference::Instance(instance));
    executor.stack.push_operand(reference);
    Ok(())
}
