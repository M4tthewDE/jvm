use std::collections::HashMap;

use class::Class;
use code::Code;
use stack::Stack;

use crate::{
    loader::ClassLoader,
    parser::constant_pool::{ClassRef, Index},
};

mod class;
mod code;
mod stack;

pub struct Executor {
    class_loader: ClassLoader,
    initialized_classes: HashMap<String, Class>,
    stack: Stack,
    pc: usize,
}

impl Executor {
    pub fn new(class_loader: ClassLoader) -> Self {
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
            stack: Stack::new(),
            pc: 0,
        }
    }

    pub fn execute(&mut self, package: &str, name: &str) {
        let class = Class::new(self.class_loader.load(package, name));
        self.execute_main_method(class);
    }

    fn execute_main_method(&mut self, class: Class) {
        let class = self.initialize(class);
        let method = class.get_main_method();

        // TODO: add []String args, see invokestatic for reference
        let code = Code::new(method.get_code_attribute().unwrap());
        self.stack.create(class, code);
        self.execute_code();
    }

    fn initialize(&mut self, class: Class) -> Class {
        if let Some(c) = self
            .initialized_classes
            .get(&key(&class.package(), &class.name()))
        {
            return c.clone();
        }

        if let Some(clinit) = class.clinit_method() {
            dbg!(clinit);
            todo!("execute clinit");
        }

        self.initialized_classes
            .insert(key(&class.package(), &class.name()), class.clone());
        class
    }

    fn execute_code(&mut self) {
        loop {
            match self.stack.get_opcode(self.pc) {
                GETSTATIC => self.execute_getstatic(),
                instruction => panic!("Unknown instruction 0x{:x}", instruction),
            }
        }
    }

    fn execute_getstatic(&mut self) {
        let field_ref_index = Index::new(self.stack.get_opcode(self.pc + 2));
        self.pc += 2;
        self.resolve_field(&field_ref_index);
        todo!("execute_getstatic");
    }

    fn resolve_field(&mut self, field_ref_index: &Index) {
        let field_ref = self.stack.field_ref(field_ref_index);
        let class = self.resolve_class(&field_ref.class_ref);
        let _field = class
            .lookup_field(&field_ref)
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
    }

    fn resolve_class(&mut self, class_ref: &ClassRef) -> Class {
        let class_file = self.class_loader.load(&class_ref.package, &class_ref.name);
        if !self.stack.can_access(&class_file) {
            panic!("{:?} is not allowed to access {class_file}, we should throw IllegalAccessError once we support exceptions", self.stack);
        }
        Class::new(class_file)
    }
}

const GETSTATIC: u8 = 0xb2;

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
