use crate::executor::{stack::Word, Executor};

pub fn aload_0(executor: &mut Executor) {
    let local_variables = executor.stack.local_variables();
    let reference = local_variables.first().unwrap();
    assert!(matches!(reference, Word::Reference { .. }));
    executor.stack.push_operand(reference.clone());
    executor.pc(1);
}
