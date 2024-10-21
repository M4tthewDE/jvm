use std::collections::HashMap;

use class::Class;
use code::Code;
use loader::ClassLoader;
use method::Method;
use op::OP_METHODS;
use stack::Stack;
use tracing::info;

use crate::{
    parser::constant_pool::{Index, MethodRef},
    ClassIdentifier, ClassName, Package,
};

mod class;
mod code;
mod field;
mod instance;
pub mod loader;
mod method;
mod native;
mod op;
mod stack;

pub struct Executor {
    class_loader: ClassLoader,
    initialized_classes: HashMap<ClassIdentifier, Class>,
    class_being_initialized: ClassIdentifier,
    stack: Stack,
    pc: usize,
}

impl Executor {
    pub fn new(class_loader: ClassLoader) -> Self {
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
            class_being_initialized: ClassIdentifier::new(Package::default(), ClassName::default()),
            stack: Stack::new(),
            pc: 0,
        }
    }

    pub fn execute(&mut self, class_identifier: ClassIdentifier) {
        let class = self.class_loader.load(class_identifier);
        self.execute_main_method(class);
    }

    fn get_class(&self, class_identifier: &ClassIdentifier) -> Option<Class> {
        self.initialized_classes.get(class_identifier).cloned()
    }

    fn execute_main_method(&mut self, class: Class) {
        self.initialize_class(class.clone());
        let class = self.get_class(&class.identifier).unwrap();
        let method = class.main_method().unwrap();

        // TODO: add []String args, see invokestatic for reference
        let code = Code::new(method.code_attribute().unwrap());
        self.stack.create(class, method, code);
        self.execute_code();
    }

    fn execute_clinit(&mut self, class: Class, method: Method) {
        let code = Code::new(method.code_attribute().unwrap());
        self.stack.create(class, method, code);
        self.execute_code();
        todo!("after execute clinit");
    }

    fn initialize_class(&mut self, class: Class) {
        if self.initialized_classes.contains_key(&class.identifier) {
            return;
        }

        if self.class_being_initialized == class.identifier {
            return;
        }

        self.class_being_initialized = class.identifier.clone();

        if let Some(clinit) = class.clinit_method() {
            self.execute_clinit(class.clone(), clinit);
        }

        self.initialized_classes
            .insert(class.identifier.clone(), class);
    }

    fn execute_code(&mut self) {
        self.pc = 0;
        loop {
            let op_code = self.stack.get_opcode(self.pc);
            info!("executing op 0x{:x}", op_code);
            let op = OP_METHODS
                .get(&op_code)
                .unwrap_or_else(|| panic!("Unknown instruction 0x{:x}", op_code));
            op(self);
        }
    }

    fn invoke_static(&mut self, method_ref: MethodRef) {
        // TODO what about the stack?
        let class = self
            .class_loader
            .load(method_ref.class.class_identifier.clone());
        self.initialize_class(class.clone());
        let method_descriptor = &method_ref
            .name_and_type
            .descriptor
            .method_descriptor()
            .unwrap();

        let operands = self.stack.pop_operands(method_descriptor.parameters.len());
        if class.is_native(&method_ref.name_and_type.name, method_descriptor) {
            if let Some(word) = native::invoke_static(
                self,
                class.identifier,
                method_ref.name_and_type.name,
                method_descriptor.parameters.clone(),
                operands,
            ) {
                self.stack.push_operand(word);
            }
        } else {
            let method = class
                .method(&method_ref.name_and_type.name, method_descriptor)
                .unwrap();
            let code = Code::new(method.code_attribute().unwrap());
            self.stack.create(class, method, code);
            self.execute_code();
            todo!("after invoke_static has executed its code");
        }
    }

    fn resolve_field(&mut self, field_index: &Index) {
        let field_ref = self.stack.field_ref(field_index);
        let class = self.resolve_class(field_ref.class_ref.class_identifier.clone());
        let _field = class
            .field(&field_ref.name_and_type)
            .unwrap_or_else(|| panic!("field {field_ref:?} not found"));
        self.initialize_class(class);
        /*
         *
         * On successful resolution of the field, the class or interface that
         * declared the resolved field is initialized if that class or interface
         * has not already been initialized (§5.5).
         * The value of the class or interface field is fetched and pushed onto
         * the operand stack.
         *
         * TODO: What does "the value" mean?
         *
         */
        todo!("")
    }

    fn resolve_class(&mut self, identifier: ClassIdentifier) -> Class {
        let class = self.class_loader.load(identifier);
        if !self.stack.can_access(&class) {
            panic!("{:?} is not allowed to access {class}, we should throw IllegalAccessError once we support exceptions", self.stack);
        }

        class
    }
}
