use std::collections::HashMap;

use stack::Stack;

use crate::{
    loader::ClassLoader,
    parser::{
        attribute::{exception::Exception, Attribute},
        class::ClassFile,
        constant_pool::{ClassRef, ConstantPool},
        method::Method,
    },
};

mod stack;

#[derive(Debug, Clone)]
pub struct Class {
    class_file: ClassFile,
}

impl Class {
    fn get_main_method(&self) -> Method {
        self.class_file.get_main_method()
    }

    fn constant_pool(&self) -> ConstantPool {
        self.class_file.constant_pool.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct Code {
    max_stacks: u16,
    max_locals: u16,
    opcodes: Vec<u8>,
    exceptions: Vec<Exception>,
    attributes: Vec<Attribute>,
}

impl Code {
    fn new(code_attribute: Attribute) -> Self {
        if let Attribute::Code {
            max_stacks,
            max_locals,
            code,
            exceptions,
            attributes,
        } = code_attribute
        {
            return Self {
                max_stacks,
                max_locals,
                opcodes: code,
                exceptions,
                attributes,
            };
        }
        panic!("can't construct Code out of {:?}", code_attribute);
    }

    fn get_opcode(&self, i: usize) -> u8 {
        self.opcodes.get(i).cloned().unwrap()
    }
}

pub struct Executor {
    class_loader: ClassLoader,
    initialized_classes: HashMap<String, Class>,
    stack: Stack,
    pc: usize,
    current_code: Code,
}

impl Executor {
    pub fn new(class_loader: ClassLoader) -> Self {
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
            stack: Stack::new(),
            pc: 0,
            current_code: Code::default(),
        }
    }

    fn initialize(&mut self, class_file: ClassFile, package: &str, name: &str) -> Class {
        let class = Class { class_file };
        self.initialized_classes
            .insert(key(package, name), class.clone());

        class
    }

    pub fn execute(&mut self, package: &str, main_class: &str) {
        let class_file = self.class_loader.get(package, main_class).unwrap();
        self.execute_main_method(class_file, package, main_class);
    }

    fn execute_main_method(&mut self, class_file: ClassFile, package: &str, main_class: &str) {
        let class = self.initialize(class_file, package, main_class);
        let method = class.get_main_method();

        // TODO: add []String args, see invokestatic for reference
        self.stack.create(class.constant_pool());
        self.current_code = Code::new(method.get_code_attribute().unwrap());
        self.execute_code();
    }

    fn execute_code(&mut self) {
        loop {
            match self.current_code.get_opcode(self.pc) {
                GETSTATIC => self.execute_getstatic(),
                instruction => panic!("Unknown instruction 0x{:x}", instruction),
            }
        }
    }

    fn execute_getstatic(&mut self) {
        let field_ref_index = self.current_code.get_opcode(self.pc + 2) as usize;
        self.pc += 2;
        self.resolve_field(field_ref_index);
        todo!();
    }

    fn resolve_field(&mut self, field_ref_index: usize) {
        let field_ref = self.stack.field_ref(field_ref_index);
        self.resolve_class(&field_ref.class_ref);
        todo!()
    }

    fn resolve_class(&mut self, class_ref: &ClassRef) {
        self.class_loader.load(&class_ref.package, &class_ref.name);
        todo!("resolve_class");
    }
}

const GETSTATIC: u8 = 0xb2;

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
