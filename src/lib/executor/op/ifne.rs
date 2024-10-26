use crate::executor::{stack::Word, Executor};

pub fn perform(executor: &mut Executor) {
    let operands = executor.stack.pop_operands(1);
    let value = operands.first().unwrap();
    if !matches!(value, Word::Int(..)) {
        panic!("value must be Int, is {value}");
    }

    dbg!(value);
    todo!();
}
