use crate::{
    parser::constant_pool::{ConstantPoolItem, Index, NameAndType},
    ClassIdentifier,
};

use super::{class::Class, code::Code, instance::Instance, method::Method};

#[derive(Debug, Clone)]
pub enum Reference {
    Instance(Instance),
    Array {
        _values: Vec<Reference>,
        _class: Class,
    },
    Null,
}

#[derive(Debug, Clone)]
pub enum Word {
    _Byte(i8),
    _Short(i16),
    Int(i32),
    _Long(i64),
    _Char(u16),
    _Float(f32),
    _Double(f64),
    _Boolean(bool),
    _ReturnAdress(usize),
    // TODO: we will need an enum that holds all the different reference types
    Reference(Reference),
    Class { _class: Class },
    _Null,
}

#[derive(Debug)]
struct Frame {
    local_variables: Vec<Word>,
    class: Class,
    method: Method,
    code: Code,
    pc: usize,
}
impl Frame {
    fn new(class: Class, method: Method, code: Code, operands: Vec<Word>) -> Self {
        Self {
            local_variables: operands,
            class,
            method,
            code,
            pc: 0,
        }
    }

    fn resolve_in_cp(&self, index: &Index) -> Option<ConstantPoolItem> {
        self.class.resolve_in_cp(index)
    }

    fn pc(&mut self, n: usize) {
        self.pc += n;
    }

    fn get_op_code(&self) -> u8 {
        self.code.get_opcode(self.pc)
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

    fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn create(&mut self, class: Class, method: Method, code: Code, operands: Vec<Word>) {
        self.frames.push(Frame::new(class, method, code, operands))
    }

    pub fn can_access(&self, class: &Class) -> bool {
        class.is_public() || class.identifier.package == self.current_frame().class.package()
    }

    pub fn get_opcode(&self) -> u8 {
        self.current_frame().get_op_code()
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

    pub fn current_method(&self) -> Method {
        self.current_frame().method.clone()
    }

    pub fn pc(&mut self, n: usize) {
        self.current_frame_mut().pc(n);
    }

    pub fn pop(&mut self) {
        self.frames.pop().unwrap();
    }

    pub fn resolve_in_cp(&self, index: &Index) -> ConstantPoolItem {
        self.current_frame().resolve_in_cp(index).unwrap()
    }

    pub fn lookup_field(&self, index: &Index) -> Option<(ClassIdentifier, NameAndType)> {
        if let ConstantPoolItem::FieldRef {
            class_identifier,
            name_and_type,
        } = self.current_frame().resolve_in_cp(index)?
        {
            Some((class_identifier, name_and_type))
        } else {
            None
        }
    }

    pub fn lookup_method(&self, index: &Index) -> Option<(ClassIdentifier, NameAndType)> {
        if let ConstantPoolItem::MethodRef {
            class_identifier,
            name_and_type,
        } = self.current_frame().resolve_in_cp(index)?
        {
            Some((class_identifier, name_and_type))
        } else {
            None
        }
    }

    pub fn lookup_class(&self, index: &Index) -> Option<ClassIdentifier> {
        if let ConstantPoolItem::ClassInfo { identifier } =
            self.current_frame().resolve_in_cp(index)?
        {
            Some(identifier)
        } else {
            None
        }
    }
}
