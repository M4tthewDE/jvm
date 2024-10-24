use crate::{executor::Executor, parser::constant_pool::Index};

pub fn perform(executor: &mut Executor) {
    executor.pc(1);
    let indexbyte1 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let indexbyte2 = executor.stack.get_opcode() as u16;
    executor.pc(1);
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    let (class_identifier, _name_and_type) = executor.stack.lookup_method(&method_index).unwrap();
    let class = executor.class_loader.load(class_identifier.clone());
    executor.initialize_class(class);
    todo!("invoke_virtual");
}
