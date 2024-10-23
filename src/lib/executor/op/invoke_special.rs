use crate::{
    executor::{code::Code, Executor},
    parser::constant_pool::Index,
};

pub fn perform(executor: &mut Executor) {
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
