use std::io::{Cursor, Read};

use super::constant_pool::ConstantPoolInfo;

#[derive(Clone, Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Clone, Debug)]
pub enum Attribute {
    Code {
        name_index: u16,
        length: u32,
        max_stacks: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table_length: u16,
        attributes: Vec<Attribute>,
    },
    LineNumberTable {
        name_index: u16,
        length: u32,
        table: Vec<LineNumberTableEntry>,
    },

    SourceFile {
        name_index: u16,
        length: u32,
        source_file_index: u16,
    },
}

impl Attribute {
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &[ConstantPoolInfo]) -> Attribute {
        let mut attribute_name_index = [0u8; 2];
        c.read_exact(&mut attribute_name_index).unwrap();
        let name_index = u16::from_be_bytes(attribute_name_index);

        let mut attribute_length = [0u8; 4];
        c.read_exact(&mut attribute_length).unwrap();
        let length = u32::from_be_bytes(attribute_length);

        let pool_info = constant_pool.get(name_index as usize).unwrap();

        if let ConstantPoolInfo::Utf { value } = pool_info {
            match value.as_str() {
                "Code" => Attribute::code(c, name_index, length, constant_pool),
                "LineNumberTable" => Attribute::line_number_table(c, name_index, length),
                "SourceFile" => Attribute::source_file(c, name_index, length),
                i => panic!("unknown attribute {i}"),
            }
        } else {
            panic!(
                "attribute_name_index must refer to Utf8 entry in constant pool, is {:?}",
                pool_info
            );
        }
    }

    fn source_file(c: &mut Cursor<&Vec<u8>>, name_index: u16, length: u32) -> Attribute {
        let mut source_file_index = [0u8; 2];
        c.read_exact(&mut source_file_index).unwrap();
        let source_file_index = u16::from_be_bytes(source_file_index);

        Attribute::SourceFile {
            name_index,
            length,
            source_file_index,
        }
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>, name_index: u16, length: u32) -> Attribute {
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

        Attribute::LineNumberTable {
            name_index,
            length,
            table,
        }
    }

    fn code(
        c: &mut Cursor<&Vec<u8>>,
        name_index: u16,
        length: u32,
        constant_pool: &[ConstantPoolInfo],
    ) -> Attribute {
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
            name_index,
            length,
            max_stacks,
            max_locals,
            code,
            exception_table_length,
            attributes,
        }
    }
}
