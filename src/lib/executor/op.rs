use std::collections::HashMap;

use crate::parser::constant_pool::Index;

use super::Executor;
use lazy_static::lazy_static;

const GETSTATIC: u8 = 0xb2;
const INVOKESTATIC: u8 = 0xb8;

type OpMethod = fn(&mut Executor);
type Op = u8;

lazy_static! {
    pub static ref OP_METHODS: HashMap<Op, OpMethod> = {
        let mut h = HashMap::new();
        h.insert(GETSTATIC, getstatic as OpMethod);
        h.insert(INVOKESTATIC, invokestatic as OpMethod);
        h
    };
}

fn invokestatic(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 2;
    let method_ref = executor.stack.method_ref(&method_index);
    executor.invoke_static(method_ref);
}

fn getstatic(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let field_ref_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 2;
    executor.resolve_field(&field_ref_index);
    todo!("execute_getstatic");
}
