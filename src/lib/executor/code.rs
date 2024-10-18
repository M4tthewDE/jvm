use crate::parser::attribute::{exception::Exception, Attribute};

#[derive(Debug, Clone, Default)]
pub struct Code {
    _max_stacks: u16,
    _max_locals: u16,
    opcodes: Vec<u8>,
    _exceptions: Vec<Exception>,
    _attributes: Vec<Attribute>,
}

impl Code {
    pub fn new(code_attribute: Attribute) -> Self {
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

    pub fn get_opcode(&self, i: usize) -> u8 {
        self.opcodes.get(i).cloned().unwrap()
    }
}
