use crate::executor::Executor;

pub fn perform(executor: &mut Executor) {
    let operands = executor.stack.pop_operands(1);
    executor.stack.push_operand(operands[0].clone());
    executor.stack.push_operand(operands[0].clone());
    executor.pc(1);
}
