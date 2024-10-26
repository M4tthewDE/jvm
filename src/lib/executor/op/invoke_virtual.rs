use crate::{
    executor::{class::Class, method::Method, Executor},
    parser::{constant_pool::Index, descriptor::FieldType},
    ClassIdentifier,
};
use anyhow::{bail, Context, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let indexbyte1 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let indexbyte2 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
    let (class_identifier, name_and_type) = executor.stack.lookup_method(&method_index)?;
    let class = executor.class_loader.load(class_identifier.clone())?;
    executor.initialize_class(class)?;
    let class = executor
        .get_class(&class_identifier)
        .context("class {class_identifier} is not initialized")?;
    let method = class.method(
        &name_and_type.name,
        &name_and_type.descriptor.method_descriptor()?,
    )?;

    if is_signature_polymorphic(&method, &class) {
        bail!("signature polymorphic methods not implemented for invokevirtual");
    }

    bail!("invokevirtual objectref??");
}

fn is_signature_polymorphic(method: &Method, class: &Class) -> bool {
    let correct_class = class.identifier
        == ClassIdentifier::from("java.lang.invoke".to_string(), "MethodHandle".to_string())
        || class.identifier
            == ClassIdentifier::from("java.lang.invoke".to_string(), "VarHandle".to_string());

    let parameters = method.descriptor.parameters.clone();
    let correct_parameters = if let Some(p) = parameters.first() {
        *p == FieldType::Array(Box::new(FieldType::Class("java.lang.Object".to_string())))
    } else {
        false
    };

    let correct_flags = method.is_varargs() && method.is_native();

    correct_class && correct_parameters && correct_flags
}
