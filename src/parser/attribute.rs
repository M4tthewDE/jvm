use std::io::{Cursor, Read, Seek};

use super::constant_pool::ConstantPoolInfo;

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
        exception_table_length: u16,
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
        let mut attribute_name_index = [0u8; 2];
        c.read_exact(&mut attribute_name_index).unwrap();
        let name_index = u16::from_be_bytes(attribute_name_index);

        c.seek_relative(4).unwrap();

        let pool_info = constant_pool.get(name_index as usize).unwrap();

        if let ConstantPoolInfo::Utf { value } = pool_info {
            match value.as_str() {
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
        let mut source_file_index = [0u8; 2];
        c.read_exact(&mut source_file_index).unwrap();
        let source_file_index = u16::from_be_bytes(source_file_index);

        Attribute::SourceFile { source_file_index }
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        let mut table_length = [0u8; 2];
        c.read_exact(&mut table_length).unwrap();
        let table_length = u16::from_be_bytes(table_length);

        let mut table = Vec::new();
        for _ in 0..table_length {
            let mut start_pc = [0u8; 2];
            c.read_exact(&mut start_pc).unwrap();
            let start_pc = u16::from_be_bytes(start_pc);

            let mut line_number = [0u8; 2];
            c.read_exact(&mut line_number).unwrap();
            let line_number = u16::from_be_bytes(line_number);

            table.push(LineNumberTableEntry {
                start_pc,
                line_number,
            });
        }

        Attribute::LineNumberTable { table }
    }

    fn code(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Attribute {
        let mut max_stacks = [0u8; 2];
        c.read_exact(&mut max_stacks).unwrap();
        let max_stacks = u16::from_be_bytes(max_stacks);

        let mut max_locals = [0u8; 2];
        c.read_exact(&mut max_locals).unwrap();
        let max_locals = u16::from_be_bytes(max_locals);

        let mut code_length = [0u8; 4];
        c.read_exact(&mut code_length).unwrap();
        let code_length = u32::from_be_bytes(code_length);

        assert!(code_length > 0);

        let mut code = vec![0u8; code_length as usize];
        c.read_exact(&mut code).unwrap();

        let mut exception_table_length = [0u8; 2];
        c.read_exact(&mut exception_table_length).unwrap();
        let exception_table_length = u16::from_be_bytes(exception_table_length);

        assert_eq!(exception_table_length, 0, "exceptions are not implemented");

        let mut attributes_count = [0u8; 2];
        c.read_exact(&mut attributes_count).unwrap();
        let attributes_count = u16::from_be_bytes(attributes_count);

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(c, constant_pool));
        }

        Attribute::Code {
            max_stacks,
            max_locals,
            code,
            exception_table_length,
            attributes,
        }
    }
}
