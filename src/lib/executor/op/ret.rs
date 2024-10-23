use crate::{executor::Executor, parser::descriptor::ReturnDescriptor};

pub fn perform(executor: &mut Executor) {
    let method = executor.stack.current_method();
    assert_eq!(method.descriptor.return_descriptor, ReturnDescriptor::Void);
    executor.stack.pop();
}
