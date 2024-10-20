use crate::parser::constant_pool::{FieldRef, Index, MethodRef};

use super::{class::Class, code::Code};

#[derive(Debug)]
pub struct Word {}

#[derive(Debug)]
struct Frame {
    _local_variables: Vec<Word>,
    class: Class,
    code: Code,
}
impl Frame {
    fn new(class: Class, code: Code) -> Self {
        Self {
            _local_variables: Vec::new(),
            class,
            code,
        }
    }

    fn field_ref(&self, field_ref_index: &Index) -> FieldRef {
        self.class.field_ref(field_ref_index).unwrap()
    }

    fn method_ref(&self, method_index: &Index) -> MethodRef {
        self.class.method_ref(method_index).unwrap()
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

    pub fn create(&mut self, class: Class, code: Code) {
        self.frames.push(Frame::new(class, code))
    }

    pub fn field_ref(&self, field_ref_index: &Index) -> FieldRef {
        self.current_frame().field_ref(field_ref_index)
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

    pub fn pop(&mut self, n: usize) -> Vec<Word> {
        let mut operands = Vec::new();
        for _ in 0..n {
            operands.push(self.operand_stack.pop().unwrap());
        }

        operands
    }
}
