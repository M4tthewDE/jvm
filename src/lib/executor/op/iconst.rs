use crate::executor::{stack::Word, Executor};

pub fn iconst_0(executor: &mut Executor) {
    executor.pc(1);
    executor.stack.push_operand(Word::Int(0));
}
