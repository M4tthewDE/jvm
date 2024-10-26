use anyhow::{bail, Result};
use std::io::Cursor;

use crate::parser::{constant_pool::Index, parse_u16, parse_u8};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotation {
    type_index: Index,
    element_value_pairs: Vec<(u16, ElementValue)>,
}

impl Annotation {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let type_index = Index::new(parse_u16(c)?);
        let num_element_value_pairs = parse_u16(c)?;

        let mut element_value_pairs = Vec::new();
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push((parse_u16(c)?, ElementValue::new(c)?));
        }

        Ok(Self {
            type_index,
            element_value_pairs,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ElementValue {
    String { const_value_index: Index },
    Boolean { const_value_index: Index },
}

impl ElementValue {
    fn new(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let tag = parse_u8(c)? as char;

        Ok(match tag {
            'Z' => ElementValue::Boolean {
                const_value_index: Index::new(parse_u16(c)?),
            },
            's' => ElementValue::String {
                const_value_index: Index::new(parse_u16(c)?),
            },
            _ => bail!("Unknown element value tag {tag}"),
        })
    }
}
