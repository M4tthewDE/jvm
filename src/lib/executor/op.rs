use std::collections::HashMap;

use crate::{
    executor::{code::Code, instance::Instance, stack::Word},
    parser::{
        constant_pool::{ConstantPoolItem, Index},
        descriptor::ReturnDescriptor,
    },
};

use super::Executor;
use lazy_static::lazy_static;

const LDC: u8 = 0x12;
const RET: u8 = 0xb1;
const GETSTATIC: u8 = 0xb2;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const NEW: u8 = 0xbb;
const DUP: u8 = 0x59;
const ALOAD_0: u8 = 0x2a;

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
        h.insert(ALOAD_0, aload_0 as OpMethod);
        h.insert(RET, ret as OpMethod);
        h.insert(LDC, ldc as OpMethod);
        h
    };
}

fn invokestatic(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    if let ConstantPoolItem::MethodRef {
        class_identifier,
        name_and_type,
    } = executor.stack.resolve_in_cp(&method_index)
    {
        executor.invoke_static(class_identifier, name_and_type);
    } else {
        panic!("no method reference found at {method_index:?}");
    }
}

fn getstatic(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let field_ref_index = Index::new((indexbyte1 << 8) | indexbyte2);
    executor.resolve_field(&field_ref_index);
    todo!("execute_getstatic");
}

fn new(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let class_index = Index::new((indexbyte1 << 8) | indexbyte2);
    if let ConstantPoolItem::ClassInfo { identifier } = executor.stack.resolve_in_cp(&class_index) {
        let class = executor.resolve_class(identifier);
        let instance = Instance::new(class);
        let reference = Word::Reference {
            _instance: instance,
        };
        executor.stack.push_operand(reference);
    } else {
        panic!("no class reference found at {class_index:?}");
    }
}

fn dup(executor: &mut Executor) {
    let operands = executor.stack.pop_operands(1);
    executor.stack.push_operand(operands[0].clone());
    executor.stack.push_operand(operands[0].clone());
    executor.pc(1);
}

fn invokespecial(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    if let ConstantPoolItem::MethodRef {
        class_identifier,
        name_and_type,
    } = executor.stack.resolve_in_cp(&method_index)
    {
        let method_descriptor = &name_and_type.descriptor.method_descriptor().unwrap();
        let class = executor.resolve_class(class_identifier);
        let method = class
            .method(&name_and_type.name, method_descriptor)
            .unwrap();
        let code = Code::new(method.code_attribute().unwrap());
        let operands = executor
            .stack
            .pop_operands(method_descriptor.parameters.len() + 1);
        executor.stack.create(class, method, code, operands);
        executor.execute_code();
        todo!("invoke_special");
    } else {
        panic!("no method reference found at {method_index:?}");
    }
}

fn aload_0(executor: &mut Executor) {
    let local_variables = executor.stack.local_variables();
    let reference = local_variables.first().unwrap();
    assert!(matches!(reference, Word::Reference { .. }));
    executor.stack.push_operand(reference.clone());
    executor.pc(1);
}

fn ret(executor: &mut Executor) {
    let method = executor.stack.current_method();
    assert_eq!(method.descriptor.return_descriptor, ReturnDescriptor::Void);
    executor.stack.pop();
}

fn ldc(executor: &mut Executor) {
    executor.pc(1);
    let index = Index::new(executor.stack.get_opcode());
    let cp_item = executor.stack.resolve_in_cp(&index);
    dbg!(cp_item);
    todo!("ldc");
}
