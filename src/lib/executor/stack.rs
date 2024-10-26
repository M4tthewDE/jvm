use anyhow::{bail, Context, Result};
use std::fmt::{Debug, Display};

use crate::{
    parser::{
        constant_pool::{ConstantPoolItem, Index, NameAndType},
        descriptor::FieldType,
    },
    ClassIdentifier,
};

use super::{class::Class, code::Code, instance::Instance, method::Method};

#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    Instance(Instance),
    Array {
        values: Vec<Reference>,
        class: Class,
    },
    Null,
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Instance(instance) => write!(f, "Instance({instance})"),
            Reference::Array { values, class } => {
                write!(f, "[] of {class} with length {}", values.len())
            }
            Reference::Null => write!(f, "Null"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    Reference(Reference),
    Class { _class: Class },
    _Null,
}
impl Word {
    pub fn from_field_type(field_type: FieldType) -> Self {
        match field_type {
            FieldType::Byte => Self::_Byte(0),
            FieldType::Char => Self::_Char(0),
            FieldType::Double => Self::_Double(0.0),
            FieldType::Float => Self::_Float(0.0),
            FieldType::Int => Self::Int(0),
            FieldType::Long => Self::_Long(0),
            FieldType::Class(_) => Self::_Null,
            FieldType::Short => Self::_Short(0),
            FieldType::Boolean => Self::_Boolean(false),
            FieldType::Array(_) => Self::_Null,
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::_Byte(val) => write!(f, "Byte({val})"),
            Word::_Short(val) => write!(f, "Short({val})"),
            Word::Int(val) => write!(f, "Int({val})"),
            Word::_Long(val) => write!(f, "Long({val})"),
            Word::_Char(val) => write!(f, "Char({val})"),
            Word::_Float(val) => write!(f, "Float({val})"),
            Word::_Double(val) => write!(f, "Double({val})"),
            Word::_Boolean(val) => write!(f, "Boolean({val})"),
            Word::_ReturnAdress(val) => write!(f, "ReturnAdress({val})"),
            Word::Reference(val) => write!(f, "Reference({val})"),
            Word::Class { _class } => write!(f, "Class({_class})"),
            Word::_Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
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

    fn resolve_in_cp(&self, index: &Index) -> Result<ConstantPoolItem> {
        self.class
            .resolve_in_cp(index)
            .context("no entry at {index:?} in constant pool")
    }

    fn pc(&mut self, n: usize) {
        self.pc += n;
    }

    fn get_op_code(&self) -> Result<u8> {
        self.code
            .get_opcode(self.pc)
            .context(format!("no op code at index {}", self.pc))
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.class, self.method.name)
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

    fn current_frame(&self) -> Result<&Frame> {
        self.frames.last().context("stack is empty")
    }

    fn current_frame_mut(&mut self) -> Result<&mut Frame> {
        self.frames.last_mut().context("stack is empty")
    }

    pub fn create(&mut self, class: Class, method: Method, code: Code, operands: Vec<Word>) {
        self.frames.push(Frame::new(class, method, code, operands));
    }

    pub fn can_access(&self, class: &Class) -> Result<bool> {
        Ok(class.is_public() || class.identifier.package == self.current_frame()?.class.package())
    }

    pub fn get_opcode(&self) -> Result<u8> {
        self.current_frame()?.get_op_code()
    }

    pub fn pop_operands(&mut self, n: usize) -> Result<Vec<Word>> {
        let mut operands = Vec::new();
        for _ in 0..n {
            operands.push(self.operand_stack.pop().context("operand stack is empty")?);
        }

        Ok(operands)
    }

    pub fn push_operand(&mut self, word: Word) {
        self.operand_stack.push(word);
    }

    pub fn local_variables(&self) -> Result<Vec<Word>> {
        Ok(self.current_frame()?.local_variables.clone())
    }

    pub fn current_method(&self) -> Result<Method> {
        Ok(self.current_frame()?.method.clone())
    }

    pub fn pc(&mut self, n: usize) -> Result<()> {
        self.current_frame_mut()?.pc(n);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn resolve_in_cp(&self, index: &Index) -> Result<ConstantPoolItem> {
        self.current_frame()?.resolve_in_cp(index)
    }

    pub fn lookup_field(&self, index: &Index) -> Result<(ClassIdentifier, NameAndType)> {
        if let ConstantPoolItem::FieldRef {
            class_identifier,
            name_and_type,
        } = self.current_frame()?.resolve_in_cp(index)?
        {
            Ok((class_identifier, name_and_type))
        } else {
            bail!("no field found for {index:?}")
        }
    }

    pub fn lookup_method(&self, index: &Index) -> Result<(ClassIdentifier, NameAndType)> {
        if let ConstantPoolItem::MethodRef {
            class_identifier,
            name_and_type,
        } = self.current_frame()?.resolve_in_cp(index)?
        {
            Ok((class_identifier, name_and_type))
        } else {
            bail!("no method found for {index:?}")
        }
    }

    pub fn lookup_class(&self, index: &Index) -> Result<ClassIdentifier> {
        if let ConstantPoolItem::ClassInfo { identifier } =
            self.current_frame()?.resolve_in_cp(index)?
        {
            Ok(identifier)
        } else {
            bail!("no class found for {index:?}")
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for frame in &self.frames {
            writeln!(f, "{frame}")?;
        }

        Ok(())
    }
}
