use std::collections::HashMap;

use crate::{
    parser::{
        constant_pool::{ClassRef, MethodRef, NameAndType},
        descriptor::{Descriptor, FieldType, MethodDescriptor, ReturnDescriptor},
    },
    ClassIdentifier,
};
use lazy_static::lazy_static;

use super::Executor;

type NativeMethod = fn(&mut Executor);

lazy_static! {
    static ref NATIVE_STATIC_METHODS: HashMap<(ClassIdentifier, String, Vec<FieldType>), NativeMethod> = {
        let mut h = HashMap::new();
        h.insert(
            (
                ClassIdentifier::from("java.lang".to_string(), "System".to_string()),
                "registerNatives".to_string(),
                vec![],
            ),
            register_natives as NativeMethod,
        );
        h
    };
}

pub fn invoke_static(
    executor: &mut Executor,
    class_identifier: ClassIdentifier,
    name: String,
    parameters: Vec<FieldType>,
) {
    NATIVE_STATIC_METHODS
        .get(&(class_identifier.clone(), name.clone(), parameters.clone()))
        .unwrap_or_else(|| {
            panic!(
        "native method {name} in {class_identifier} with parameters {parameters:?} not implemented"
    )
        })(executor);
}

fn register_natives(executor: &mut Executor) {
    let method_ref = MethodRef {
        class: ClassRef {
            class_identifier: ClassIdentifier::from("java.lang".to_string(), "System".to_string()),
        },
        name_and_type: NameAndType {
            name: "initPhase1".to_string(),
            descriptor: Descriptor::Method(MethodDescriptor {
                parameters: vec![],
                return_descriptor: ReturnDescriptor::Void,
            }),
        },
    };
    executor.invoke_static(method_ref);
}
