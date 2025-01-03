use std::collections::HashMap;

use crate::{
    parser::{
        constant_pool::NameAndType,
        descriptor::{Descriptor, FieldType, MethodDescriptor, ReturnDescriptor},
    },
    ClassIdentifier,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;

use super::{stack::Word, Executor};

type NativeMethod = fn(&mut Executor, Vec<Word>) -> Result<Option<Word>>;

lazy_static! {
    static ref NATIVE_STATIC_METHODS: HashMap<(ClassIdentifier, String, Vec<FieldType>), NativeMethod> = {
        let mut h = HashMap::new();
        h.insert(
            (
                ClassIdentifier::from("java.lang".to_string(), "System".to_string()),
                "registerNatives".to_string(),
                vec![],
            ),
            register_natives_system as NativeMethod,
        );
        h.insert(
            (
                ClassIdentifier::from("java.lang".to_string(), "Class".to_string()),
                "registerNatives".to_string(),
                vec![],
            ),
            register_natives_class as NativeMethod,
        );
        h
    };
}

pub fn invoke_static(
    executor: &mut Executor,
    class_identifier: ClassIdentifier,
    name: String,
    parameters: Vec<FieldType>,
    operands: Vec<Word>,
) -> Result<Option<Word>> {
    NATIVE_STATIC_METHODS
        .get(&(class_identifier.clone(), name.clone(), parameters.clone()))
        .context(format!(
        "native method {name} in {class_identifier} with parameters {parameters:?} not implemented"
    ))?(executor, operands)
}

fn register_natives_system(executor: &mut Executor, _operands: Vec<Word>) -> Result<Option<Word>> {
    let class_identifier = ClassIdentifier::from("java.lang".to_string(), "System".to_string());
    let name_and_type = NameAndType {
        name: "initPhase1".to_string(),
        descriptor: Descriptor::Method(MethodDescriptor {
            parameters: vec![],
            return_descriptor: ReturnDescriptor::Void,
        }),
    };
    executor.invoke_static(class_identifier, name_and_type)?;
    Ok(None)
}

fn register_natives_class(_executor: &mut Executor, _operands: Vec<Word>) -> Result<Option<Word>> {
    Ok(None)
}
