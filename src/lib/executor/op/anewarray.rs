use crate::{
    executor::{
        stack::{Reference, Word},
        Executor,
    },
    parser::constant_pool::{ConstantPoolItem, Index},
};
use anyhow::{bail, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    let operands = executor.stack.pop_operands(1)?;
    match operands.first().unwrap() {
        Word::Int(count) => {
            executor.pc(1)?;
            let indexbyte1 = executor.stack.get_opcode()? as u16;
            executor.pc(1)?;
            let indexbyte2 = executor.stack.get_opcode()? as u16;
            executor.pc(1)?;
            let index = Index::new((indexbyte1 << 8) | indexbyte2);
            let cp_item = executor.stack.resolve_in_cp(&index)?;

            match cp_item {
                ConstantPoolItem::ClassInfo { identifier } => {
                    let class = executor.resolve_class(identifier)?;
                    let reference = Word::Reference(Reference::Array {
                        values: vec![Reference::Null; *count as usize],
                        class,
                    });
                    executor.stack.push_operand(reference);
                    Ok(())
                }
                _ => bail!("Unsupported constant pool item {cp_item:?}"),
            }
        }
        o => bail!("array count has to be Int, is {}", o),
    }
}
