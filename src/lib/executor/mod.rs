use std::{collections::HashMap, fmt::Display};

use stack::Stack;

use crate::{
    loader::ClassLoader,
    parser::{
        attribute::{exception::Exception, Attribute},
        class::ClassFile,
        constant_pool::{ClassRef, ConstantPool, Index},
        method::Method,
    },
};

mod stack;

#[derive(Debug, Clone)]
pub struct Class {
    class_file: ClassFile,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_file)
    }
}

impl Class {
    fn get_main_method(&self) -> Method {
        self.class_file.get_main_method()
    }

    fn constant_pool(&self) -> ConstantPool {
        self.class_file.constant_pool.clone()
    }

    fn can_access(&self, class: &ClassFile) -> bool {
        class.is_public() || class.package == self.class_file.package
    }

    fn lookup_field(&self) {
        todo!("lookup_field")
    }
}

#[derive(Debug, Clone, Default)]
struct Code {
    _max_stacks: u16,
    _max_locals: u16,
    opcodes: Vec<u8>,
    _exceptions: Vec<Exception>,
    _attributes: Vec<Attribute>,
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
                _max_stacks: max_stacks,
                _max_locals: max_locals,
                opcodes: code,
                _exceptions: exceptions,
                _attributes: attributes,
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
    current_class: Class,
    stack: Stack,
    pc: usize,
    current_code: Code,
}

impl Executor {
    pub fn new(mut class_loader: ClassLoader, package: &str, name: &str) -> Self {
        let current_class = Class {
            class_file: class_loader.load(package, name),
        };
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
            current_class,
            stack: Stack::new(),
            pc: 0,
            current_code: Code::default(),
        }
    }

    pub fn execute(&mut self) {
        self.execute_main_method();
    }

    fn execute_main_method(&mut self) {
        let class = self.initialize(self.current_class.clone().class_file);
        let method = class.get_main_method();

        // TODO: add []String args, see invokestatic for reference
        self.stack.create(class.constant_pool());
        self.current_code = Code::new(method.get_code_attribute().unwrap());
        self.execute_code();
    }

    fn initialize(&mut self, class_file: ClassFile) -> Class {
        let class = Class {
            class_file: class_file.clone(),
        };
        self.initialized_classes.insert(
            key(&class.class_file.package, &class.class_file.name),
            class.clone(),
        );

        class
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
        let field_ref_index = Index::new(self.current_code.get_opcode(self.pc + 2));
        self.pc += 2;
        self.resolve_field(&field_ref_index);
        todo!();
    }

    fn resolve_field(&mut self, field_ref_index: &Index) {
        let field_ref = self.stack.field_ref(field_ref_index);
        let class = self.resolve_class(&field_ref.class_ref);
        class.lookup_field();

        todo!("resolve_field");
    }

    fn resolve_class(&mut self, class_ref: &ClassRef) -> Class {
        let class_file = self.class_loader.load(&class_ref.package, &class_ref.name);
        self.apply_access_control(&class_file);
        Class { class_file }
    }

    fn apply_access_control(&self, class: &ClassFile) {
        if !self.current_class.can_access(class) {
            panic!("{} is not allowed to access {}, we should throw IllegalAccessError once we support exceptions", self.current_class, class);
        }
    }
}

const GETSTATIC: u8 = 0xb2;

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
