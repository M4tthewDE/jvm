use std::io::Cursor;

use crate::parser::{parse_u16, parse_u8};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<(u16, ElementValue)>,
}

impl Annotation {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        let type_index = parse_u16(c);
        let num_element_value_pairs = parse_u16(c);

        let mut element_value_pairs = Vec::new();
        for _ in 0..num_element_value_pairs {
            let element_name_index = parse_u16(c);
            let element_value = ElementValue::new(c);
            element_value_pairs.push((element_name_index, element_value));
        }

        Self {
            type_index,
            element_value_pairs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ElementValue {
    String { const_value_index: u16 },
    Boolean { const_value_index: u16 },
}

impl ElementValue {
    fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        let tag = parse_u8(c) as char;

        match tag {
            'Z' => ElementValue::Boolean {
                const_value_index: parse_u16(c),
            },
            's' => ElementValue::String {
                const_value_index: parse_u16(c),
            },
            _ => panic!("Unknown element value tag {tag}"),
        }
    }
}
