use crate::parser::constant_pool::{ConstantPool, FieldRef};

#[derive(Debug)]
struct Word {}

#[derive(Debug)]
struct Frame {
    _local_variables: Vec<Word>,
    constant_pool: ConstantPool,
}
impl Frame {
    fn new(constant_pool: ConstantPool) -> Self {
        Self {
            _local_variables: Vec::new(),
            constant_pool,
        }
    }

    fn field_ref(&self, field_ref_index: usize) -> FieldRef {
        self.constant_pool.field_ref(field_ref_index).unwrap()
    }
}

#[derive(Debug)]
pub struct Stack {
    frames: Vec<Frame>,
    _operand_stack: Vec<Word>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            _operand_stack: Vec::new(),
        }
    }

    pub fn create(&mut self, constant_pool: ConstantPool) {
        self.frames.push(Frame::new(constant_pool))
    }

    pub fn field_ref(&self, field_ref_index: usize) -> FieldRef {
        self.frames.last().unwrap().field_ref(field_ref_index)
    }
}
