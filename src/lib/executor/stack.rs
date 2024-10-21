use crate::parser::constant_pool::{ClassRef, FieldRef, Index, MethodRef};

use super::{class::Class, code::Code, instance::Instance, method::Method};

#[derive(Debug, Clone)]
pub enum Word {
    _Byte(i8),
    _Short(i16),
    _Int(i32),
    _Long(i64),
    _Char(u16),
    _Float(f32),
    _Double(f64),
    _Boolean(bool),
    _ReturnAdress(usize),
    Reference { _instance: Instance },
    _Null,
}

#[derive(Debug)]
struct Frame {
    local_variables: Vec<Word>,
    class: Class,
    _method: Method,
    code: Code,
}
impl Frame {
    fn new(class: Class, method: Method, code: Code, operands: Vec<Word>) -> Self {
        Self {
            local_variables: operands,
            class,
            _method: method,
            code,
        }
    }

    fn field_ref(&self, field_ref_index: &Index) -> FieldRef {
        self.class.field_ref(field_ref_index).unwrap()
    }

    fn method_ref(&self, method_index: &Index) -> MethodRef {
        self.class.method_ref(method_index).unwrap()
    }

    fn class_ref(&self, class_index: &Index) -> ClassRef {
        self.class.class_ref(class_index).unwrap()
    }
}

#[derive(Debug)]
pub struct Stack {
    frames: Vec<Frame>,
    operand_stack: Vec<Word>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            operand_stack: Vec::new(),
        }
    }

    fn current_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    pub fn create(&mut self, class: Class, method: Method, code: Code, operands: Vec<Word>) {
        self.frames.push(Frame::new(class, method, code, operands))
    }

    pub fn field_ref(&self, field_ref_index: &Index) -> FieldRef {
        self.current_frame().field_ref(field_ref_index)
    }

    pub fn class_ref(&self, class_index: &Index) -> ClassRef {
        self.current_frame().class_ref(class_index)
    }

    pub fn method_ref(&self, method_index: &Index) -> MethodRef {
        self.current_frame().method_ref(method_index)
    }

    pub fn can_access(&self, class: &Class) -> bool {
        class.is_public() || class.identifier.package == self.current_frame().class.package()
    }

    pub fn get_opcode(&self, i: usize) -> u8 {
        self.current_frame().code.get_opcode(i)
    }

    pub fn pop_operands(&mut self, n: usize) -> Vec<Word> {
        let mut operands = Vec::new();
        for _ in 0..n {
            operands.push(self.operand_stack.pop().unwrap());
        }

        operands
    }

    pub fn push_operand(&mut self, word: Word) {
        self.operand_stack.push(word);
    }

    pub fn local_variables(&self) -> Vec<Word> {
        self.current_frame().local_variables.clone()
    }
}
