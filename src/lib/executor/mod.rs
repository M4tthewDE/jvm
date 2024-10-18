use std::collections::HashMap;

use class::Class;
use code::Code;
use method::Method;
use stack::Stack;

use crate::{
    loader::ClassLoader,
    parser::constant_pool::{ClassRef, Index},
    ClassIdentifier, ClassName, Package,
};

// TODO: restrict after loader is in executor
pub mod class;
mod code;
mod field;
mod method;
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
        self.initialize(class.clone());
        let class = self.get_class(&class.identifier).unwrap();
        let method = class.main_method();

        // TODO: add []String args, see invokestatic for reference
        let code = Code::new(method.get_code_attribute().unwrap());
        self.stack.create(class, code);
        self.execute_code();
    }

    fn execute_clinit(&mut self, class: Class, method: &Method) {
        let code = Code::new(method.get_code_attribute().unwrap());
        self.stack.create(class, code);
        self.execute_code();
        todo!("after execute clinit");
    }

    fn initialize(&mut self, class: Class) {
        if self.initialized_classes.contains_key(&class.identifier) {
            return;
        }

        if self.class_being_initialized == class.identifier {
            return;
        }

        self.class_being_initialized = class.identifier.clone();

        if let Some(clinit) = &class.clinit_method() {
            self.execute_clinit(class.clone(), clinit);
        }

        self.initialized_classes
            .insert(class.identifier.clone(), class);
    }

    fn execute_code(&mut self) {
        self.pc = 0;
        loop {
            match self.stack.get_opcode(self.pc) {
                GETSTATIC => self.getstatic(),
                INVOKESTATIC => self.invoke_static(),
                instruction => panic!("Unknown instruction 0x{:x}", instruction),
            }
        }
    }

    fn invoke_static(&mut self) {
        let indexbyte1 = self.stack.get_opcode(self.pc + 1) as u16;
        let indexbyte2 = self.stack.get_opcode(self.pc + 2) as u16;
        let method_index = Index::new((indexbyte1 << 8) | indexbyte2);
        self.pc += 2;
        let method_ref = self.stack.method_ref(&method_index);
        let class = self
            .class_loader
            .load(method_ref.class.class_identifier.clone());
        self.initialize(class.clone());
        if class.is_native(&method_ref) {
            todo!("implement invoke_static for native methods");
        } else {
            todo!("implement invoke_static for non-native methods");
        }
    }

    fn getstatic(&mut self) {
        let indexbyte1 = self.stack.get_opcode(self.pc + 1) as u16;
        let indexbyte2 = self.stack.get_opcode(self.pc + 2) as u16;
        let field_ref_index = Index::new((indexbyte1 << 8) | indexbyte2);
        self.pc += 2;
        self.resolve_field(&field_ref_index);
        todo!("execute_getstatic");
    }

    fn resolve_field(&mut self, field_ref_index: &Index) {
        let field_ref = self.stack.field_ref(field_ref_index);
        let class = self.resolve_class(&field_ref.class_ref);
        let _field = class
            .field(&field_ref.name_and_type)
            .unwrap_or_else(|| panic!("field {field_ref:?} not found"));
        self.initialize(class);
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

    fn resolve_class(&mut self, class_ref: &ClassRef) -> Class {
        let class = self.class_loader.load(class_ref.class_identifier.clone());
        if !self.stack.can_access(&class) {
            panic!("{:?} is not allowed to access {class}, we should throw IllegalAccessError once we support exceptions", self.stack);
        }

        class
    }
}

const GETSTATIC: u8 = 0xb2;
const INVOKESTATIC: u8 = 0xb8;
