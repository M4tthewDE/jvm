use crate::{
    executor::{stack::Word, Executor},
    parser::constant_pool::{ConstantPoolItem, Index},
};

pub fn perform(executor: &mut Executor) {
    executor.pc(1);
    let index = Index::new(executor.stack.get_opcode());
    let cp_item = executor.stack.resolve_in_cp(&index);

    assert!(
        !matches!(cp_item, ConstantPoolItem::Long { .. }),
        "constant pool item cannot be of type Long"
    );

    match cp_item {
        ConstantPoolItem::Reserved => {
            panic!("constant pool item reserved should never appear here")
        }

        ConstantPoolItem::ClassInfo { identifier } => {
            let class = executor.resolve_class(identifier);
            executor.stack.push_operand(Word::Class { _class: class })
        }
        _ => panic!("constant pool item {cp_item:?} not yet supported by ldc"),
    }
}
