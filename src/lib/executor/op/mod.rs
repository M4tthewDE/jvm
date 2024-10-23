use crate::{
    executor::{code::Code, instance::Instance, stack::Word},
    parser::{constant_pool::Index, descriptor::ReturnDescriptor},
};

use super::Executor;

mod invoke_static;

const LDC: u8 = 0x12;
const RET: u8 = 0xb1;
const GETSTATIC: u8 = 0xb2;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const NEW: u8 = 0xbb;
const DUP: u8 = 0x59;
const ALOAD_0: u8 = 0x2a;

type OpMethod = fn(&mut Executor);

pub fn get_op(op_code: &u8) -> Option<OpMethod> {
    match *op_code {
        INVOKESTATIC => Some(invoke_static::perform as OpMethod),
        GETSTATIC => Some(getstatic as OpMethod),
        INVOKESPECIAL => Some(invokespecial as OpMethod),
        NEW => Some(new as OpMethod),
        DUP => Some(dup as OpMethod),
        ALOAD_0 => Some(aload_0 as OpMethod),
        RET => Some(ret as OpMethod),
        LDC => Some(ldc as OpMethod),
        _ => None,
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

    let identifier = executor.stack.lookup_class(&class_index).unwrap();
    let class = executor.resolve_class(identifier);
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
    executor.pc(1);
}

fn invokespecial(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);

    let (class_identifier, name_and_type) = executor.stack.lookup_method(&method_index).unwrap();
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
    let _cp_item = executor.stack.resolve_in_cp(&index);
    todo!("ldc");
}
