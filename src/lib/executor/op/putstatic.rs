use tracing::debug;

use crate::{
    executor::{stack::Word, Executor},
    parser::{
        constant_pool::Index,
        descriptor::{Descriptor, FieldType},
    },
};
use anyhow::{bail, Context, Result};

pub fn perform(executor: &mut Executor) -> Result<()> {
    executor.pc(1)?;
    let indexbyte1 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let indexbyte2 = executor.stack.get_opcode()? as u16;
    executor.pc(1)?;
    let field_index = Index::new((indexbyte1 << 8) | indexbyte2);
    let field = executor.resolve_field(&field_index)?;
    let operands = executor.stack.pop_operands(1)?;
    let value = operands.first().context("local variables are empty")?;

    if !is_compatible(&field.descriptor, value) {
        bail!("{field} cannot be set to {value}");
    }

    executor.assign_static_field(&field, value)
}

fn is_compatible(descriptor: &Descriptor, value: &Word) -> bool {
    if let Descriptor::Field(field_type) = descriptor {
        match field_type {
            FieldType::Byte => matches!(value, Word::_Byte(..)),
            FieldType::Char => matches!(value, Word::_Char(..)),
            FieldType::Double => matches!(value, Word::_Double(..)),
            FieldType::Float => matches!(value, Word::_Float(..)),
            FieldType::Int => matches!(value, Word::Int(..)),
            FieldType::Long => matches!(value, Word::_Long(..)),
            FieldType::Short => matches!(value, Word::_Short(..)),
            FieldType::Boolean => matches!(value, Word::_Boolean(..)),
            _ => {
                debug!("Assignment compatibility is not checked for reference types!");
                true
            }
        }
    } else {
        false
    }
}
