use crate::{
    executor::{stack::Word, Executor},
    parser::constant_pool::{ConstantPoolItem, Index},
};
use anyhow::{bail, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let index = Index::new(executor.stack.get_opcode()?);
    executor.pc(1)?;
    let cp_item = executor.stack.resolve_in_cp(&index)?;

    if matches!(cp_item, ConstantPoolItem::Long { .. }) {
        bail!("constant pool item cannot be of type Long");
    }

    match cp_item {
        ConstantPoolItem::Reserved => {
            bail!("constant pool item reserved should never appear here")
        }

        ConstantPoolItem::ClassInfo { identifier } => {
            let class = executor.resolve_class(identifier)?;
            executor.stack.push_operand(Word::Class { _class: class })
        }
        _ => bail!("constant pool item {cp_item:?} not yet supported by ldc"),
    }

    Ok(())
}
