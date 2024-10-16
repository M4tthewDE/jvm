use std::io::{Cursor, Seek};

use annotation::Annotation;

use crate::parser::{parse_u32, parse_vec};

use super::{
    constant_pool::{ConstantPool, ConstantPoolInfo},
    parse_u16,
};

mod annotation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumberTableEntry {
    fn new(c: &mut Cursor<&Vec<u8>>) -> LineNumberTableEntry {
        LineNumberTableEntry {
            start_pc: parse_u16(c),
            line_number: parse_u16(c),
        }
    }
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
    ConstantValue {
        constant_value_index: u16,
    },
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
    },
}

impl Attribute {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Attribute {
        let name_index = parse_u16(c) as usize;
        c.seek_relative(4).unwrap();

        match Attribute::get_text(constant_pool, name_index).as_str() {
            "Code" => Attribute::code(c, constant_pool),
            "LineNumberTable" => Attribute::line_number_table(c),
            "SourceFile" => Attribute::source_file(c),
            "ConstantValue" => Attribute::constant_value(c),
            "RuntimeVisibleAnnotations" => Attribute::runtime_visible_annotations(c),
            i => panic!("unknown attribute {i}"),
        }
    }

    fn get_text(constant_pool: &ConstantPool, name_index: usize) -> String {
        let pool_info = constant_pool.infos.get(name_index).unwrap();
        if let ConstantPoolInfo::Utf { text } = pool_info {
            text.to_string()
        } else {
            panic!(
                "attribute_name_index must refer to Utf8 entry in constant pool, is {:?}",
                pool_info
            );
        }
    }

    pub fn attributes(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Vec<Attribute> {
        let attributes_count = parse_u16(c) as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }
        attributes
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
            table.push(LineNumberTableEntry::new(c));
        }
        Attribute::LineNumberTable { table }
    }

    fn code(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Attribute {
        let max_stacks = parse_u16(c);
        let max_locals = parse_u16(c);

        let code_length = parse_u32(c) as usize;
        assert!(code_length > 0);
        let code = parse_vec(c, code_length);

        let exception_table_length = parse_u16(c);
        assert_eq!(exception_table_length, 0, "exceptions are not implemented");

        Attribute::Code {
            max_stacks,
            max_locals,
            code,
            attributes: Attribute::attributes(c, constant_pool),
        }
    }

    fn constant_value(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        Attribute::ConstantValue {
            constant_value_index: parse_u16(c),
        }
    }

    fn runtime_visible_annotations(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        let num_annotations = parse_u16(c);

        let mut annotations = Vec::new();
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(c));
        }

        Attribute::RuntimeVisibleAnnotations { annotations }
    }
}
