use std::io::{Cursor, Seek};

use crate::parser::{parse_u32, parse_vec};

use super::{constant_pool::ConstantPoolInfo, parse_u16};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribute {
    Code {
        max_stacks: u16,
        max_locals: u16,
        code: Vec<u8>,
        attributes: Vec<Attribute>,
    },
    LineNumberTable {
        table: Vec<LineNumberTableEntry>,
    },

    SourceFile {
        source_file_index: u16,
    },
}

impl Attribute {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Attribute {
        let name_index = parse_u16(c);
        c.seek_relative(4).unwrap();
        let pool_info = constant_pool.get(name_index as usize).unwrap();

        if let ConstantPoolInfo::Utf { text } = pool_info {
            match text.as_str() {
                "Code" => Attribute::code(c, constant_pool),
                "LineNumberTable" => Attribute::line_number_table(c),
                "SourceFile" => Attribute::source_file(c),
                i => panic!("unknown attribute {i}"),
            }
        } else {
            panic!(
                "attribute_name_index must refer to Utf8 entry in constant pool, is {:?}",
                pool_info
            );
        }
    }

    fn source_file(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        Attribute::SourceFile {
            source_file_index: parse_u16(c),
        }
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        let table_length = parse_u16(c) as usize;

        let mut table = Vec::with_capacity(table_length);
        for _ in 0..table_length {
            table.push(LineNumberTableEntry {
                start_pc: parse_u16(c),
                line_number: parse_u16(c),
            });
        }

        Attribute::LineNumberTable { table }
    }

    fn code(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Attribute {
        let max_stacks = parse_u16(c);
        let max_locals = parse_u16(c);

        let code_length = parse_u32(c) as usize;
        assert!(code_length > 0);
        let code = parse_vec(c, code_length);

        let exception_table_length = parse_u16(c);
        assert_eq!(exception_table_length, 0, "exceptions are not implemented");

        let attributes_count = parse_u16(c) as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }

        Attribute::Code {
            max_stacks,
            max_locals,
            code,
            attributes,
        }
    }
}
