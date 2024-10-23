use crate::{executor::Executor, parser::constant_pool::Index};

pub fn perform(executor: &mut Executor) {
    executor.pc(1);
    let index = Index::new(executor.stack.get_opcode());
    let _cp_item = executor.stack.resolve_in_cp(&index);
    todo!("ldc");
}
