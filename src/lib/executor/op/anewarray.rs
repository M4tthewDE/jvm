use crate::{
    executor::{
        stack::{Reference, Word},
        Executor,
    },
    parser::constant_pool::{ConstantPoolItem, Index},
};

pub fn perform(executor: &mut Executor) {
    let operands = executor.stack.pop_operands(1);
    if let Word::Int(count) = operands.first().unwrap() {
        executor.pc(1);
        let indexbyte1 = executor.stack.get_opcode() as u16;
        executor.pc(1);
        let indexbyte2 = executor.stack.get_opcode() as u16;
        executor.pc(1);
        let index = Index::new((indexbyte1 << 8) | indexbyte2);
        let cp_item = executor.stack.resolve_in_cp(&index);

        match cp_item {
            ConstantPoolItem::ClassInfo { identifier } => {
                let class = executor.resolve_class(identifier);
                let reference = Word::Reference(Reference::Array {
                    _values: vec![Reference::Null; *count as usize],
                    _class: class,
                });
                executor.stack.push_operand(reference);
            }
            _ => panic!("Unsupported constant pool item {cp_item:?}"),
        }
    }
}
