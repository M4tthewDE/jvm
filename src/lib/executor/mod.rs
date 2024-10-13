use std::collections::HashMap;

use crate::{
    loader::ClassLoader,
    parser::{class::ClassFile, method::Method},
};

#[derive(Debug, Clone)]
pub struct Class {
    class_file: ClassFile,
}

impl Class {
    fn get_main_method(&self) -> Method {
        self.class_file.get_main_method()
    }
}

pub struct Executor {
    class_loader: ClassLoader,
    initialized_classes: HashMap<String, Class>,
}

impl Executor {
    pub fn new(class_loader: ClassLoader) -> Self {
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
        }
    }

    pub fn execute(&mut self, package: &str, main_class: &str) {
        let class_file = self.class_loader.get(package, main_class).unwrap();
        self.execute_main_method(class_file, package, main_class);
    }

    fn execute_main_method(&mut self, class_file: ClassFile, package: &str, main_class: &str) {
        let class = self.initialize(class_file, package, main_class);
        let method = class.get_main_method();
        self.execute_method(&method);
    }

    fn execute_method(&self, method: &Method) {
        dbg!(method);
        // If the method is not native, the nargs words of arguments are
        // popped from the operand stack. A new stack frame is created for
        // the method being invoked, and the words of arguments are made
        // the values of its first nargs local variables, with arg1 in local vari-
        // able 0, arg2 in local variable 1, and so on. The new stack frame is
        // then made current, and the Java Virtual Machine pc is set to the
        // opcode of the first instruction of the method to be invoked. Execu-
        // tion continues with the first instruction of the method.
        //
        // TODO:
        //  - frames (3.6)
        todo!()
    }

    fn initialize(&mut self, class_file: ClassFile, package: &str, name: &str) -> Class {
        let class = Class { class_file };
        self.initialized_classes
            .insert(key(package, name), class.clone());

        class
    }
}

fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}
