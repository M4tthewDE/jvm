use crate::parser::attribute::Attribute;

#[derive(Debug, Clone, Default)]
pub struct Code {
    opcodes: Vec<u8>,
}

impl Code {
    pub fn new(code_attribute: Attribute) -> Self {
        if let Attribute::Code {
            max_stacks: _,
            max_locals: _,
            code,
            exceptions: _,
            attributes: _,
        } = code_attribute
        {
            return Self { opcodes: code };
        }
        panic!("can't construct Code out of {:?}", code_attribute);
    }

    pub fn get_opcode(&self, i: usize) -> Option<u8> {
        self.opcodes.get(i).cloned()
    }
}
