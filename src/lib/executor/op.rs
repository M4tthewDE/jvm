use std::collections::HashMap;

use crate::{
    executor::{instance::Instance, stack::Word},
    parser::constant_pool::Index,
};

use super::Executor;
use lazy_static::lazy_static;

const GETSTATIC: u8 = 0xb2;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const NEW: u8 = 0xbb;
const DUP: u8 = 0x59;

type OpMethod = fn(&mut Executor);
type Op = u8;

lazy_static! {
    pub static ref OP_METHODS: HashMap<Op, OpMethod> = {
        let mut h = HashMap::new();
        h.insert(GETSTATIC, getstatic as OpMethod);
        h.insert(INVOKESPECIAL, invokespecial as OpMethod);
        h.insert(INVOKESTATIC, invokestatic as OpMethod);
        h.insert(NEW, new as OpMethod);
        h.insert(DUP, dup as OpMethod);
        h
    };
}

fn invokestatic(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 3;
    let method_ref = executor.stack.method_ref(&method_index);
    executor.invoke_static(method_ref);
}

fn getstatic(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let field_ref_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 3;
    executor.resolve_field(&field_ref_index);
    todo!("execute_getstatic");
}

fn new(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let class_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 3;
    let class_ref = executor.stack.class_ref(&class_index);
    let class = executor.resolve_class(class_ref.class_identifier.clone());
    let instance = Instance::new(class);
    let reference = Word::Reference {
        _instance: instance,
    };
    executor.stack.push_operand(reference);
}

fn dup(executor: &mut Executor) {
    let operands = executor.stack.pop_operands(1);
    executor.stack.push_operand(operands[0].clone());
    executor.stack.push_operand(operands[0].clone());
    executor.pc += 1;
}

fn invokespecial(executor: &mut Executor) {
    let indexbyte1 = executor.stack.get_opcode(executor.pc + 1) as u16;
    let indexbyte2 = executor.stack.get_opcode(executor.pc + 2) as u16;
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.pc += 3;
    let method_ref = executor.stack.method_ref(&method_index);
    dbg!(&method_ref);
    todo!("invoke_special");
}
