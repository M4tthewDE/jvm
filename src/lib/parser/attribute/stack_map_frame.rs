use std::io::Cursor;

use crate::parser::{constant_pool::Index, parse_u16, parse_u8};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StackMapFrame {
    SameFrame {
        offset_delta: u8,
    },
    SameLocals {
        offset_delta: u8,
        verification_type: VerificationType,
    },
    Chop {
        offset_delta: u16,
    },
    Append {
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },
    Full {
        offset_delta: u16,
        locals: Vec<VerificationType>,
        stack_items: Vec<VerificationType>,
    },
}

impl StackMapFrame {
    pub fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        let tag = parse_u8(c);
        match tag {
            0..=63 => Self::SameFrame { offset_delta: tag },
            64..=127 => Self::same_locals(c, tag),
            248..=250 => Self::chop(c),
            252..=254 => Self::append(c, tag),
            255 => Self::full(c),
            _ => panic!("invalid stack map frame tag {tag}"),
        }
    }

    fn same_locals(c: &mut Cursor<&Vec<u8>>, tag: u8) -> StackMapFrame {
        Self::SameLocals {
            offset_delta: tag - 64,
            verification_type: VerificationType::new(c),
        }
    }

    fn chop(c: &mut Cursor<&Vec<u8>>) -> StackMapFrame {
        Self::Chop {
            offset_delta: parse_u16(c),
        }
    }

    fn append(c: &mut Cursor<&Vec<u8>>, tag: u8) -> StackMapFrame {
        let offset_delta = parse_u16(c);
        let mut locals = Vec::new();
        for _ in 0..tag - 251 {
            locals.push(VerificationType::new(c));
        }

        Self::Append {
            offset_delta,
            locals,
        }
    }

    fn full(c: &mut Cursor<&Vec<u8>>) -> StackMapFrame {
        let offset_delta = parse_u16(c);

        let number_of_locals = parse_u16(c) as usize;
        let mut locals = Vec::with_capacity(number_of_locals);
        for _ in 0..number_of_locals {
            locals.push(VerificationType::new(c));
        }

        let number_of_stack_items = parse_u16(c) as usize;
        let mut stack_items = Vec::with_capacity(number_of_stack_items);
        for _ in 0..number_of_stack_items {
            stack_items.push(VerificationType::new(c));
        }

        StackMapFrame::Full {
            offset_delta,
            locals,
            stack_items,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerificationType {
    Integer,
    ConstantClass { cpoll_index: Index },
}

impl VerificationType {
    fn new(c: &mut Cursor<&Vec<u8>>) -> Self {
        let tag = parse_u8(c);
        match tag {
            1 => VerificationType::Integer,
            7 => VerificationType::ConstantClass {
                cpoll_index: Index::new(parse_u16(c)),
            },
            _ => panic!("invalid verification type tag {tag}"),
        }
    }
}
