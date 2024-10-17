use std::io::{Cursor, Seek};

use annotation::Annotation;
use bootstrap_method::BootstrapMethod;
use exception::Exception;
use inner_class::InnerClass;
use local_variable::{LocalVariable, LocalVariableType};
use stack_map_frame::StackMapFrame;
use tracing::{info, instrument};

use crate::parser::{parse_u32, parse_vec};

use super::{
    constant_pool::{ConstantPool, ConstantPoolInfo},
    parse_u16,
};

mod annotation;
mod bootstrap_method;
pub mod exception;
mod inner_class;
mod local_variable;
mod stack_map_frame;

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
        exceptions: Vec<Exception>,
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
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableType>,
    },
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    Signature {
        signature_index: u16,
    },
    Deprecated,
    NestMembers {
        classes: Vec<u16>,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },

    InnerClasses {
        classes: Vec<InnerClass>,
    },
}

impl Attribute {
    #[instrument(skip_all, name = "attribute")]
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Attribute {
        let name_index = parse_u16(c) as usize;
        c.seek_relative(4).unwrap();

        info!("parsing attribute {name_index}");
        let text = Attribute::get_text(constant_pool, name_index);
        info!("parsing attribute {text}");

        match text.as_str() {
            "Code" => Attribute::code(c, constant_pool),
            "LineNumberTable" => Attribute::line_number_table(c),
            "SourceFile" => Attribute::source_file(c),
            "ConstantValue" => Attribute::constant_value(c),
            "RuntimeVisibleAnnotations" => Attribute::runtime_visible_annotations(c),
            "LocalVariableTable" => Attribute::local_variable_table(c),
            "StackMapTable" => Attribute::stack_map_table(c),
            "Exceptions" => Attribute::exceptions(c),
            "LocalVariableTypeTable" => Attribute::local_variable_type_table(c),
            "Signature" => Attribute::signature(c),
            "Deprecated" => Attribute::Deprecated,
            "NestMembers" => Attribute::nest_members(c),
            "BootstrapMethods" => Attribute::bootstrap_methods(c),
            "InnerClasses" => Attribute::inner_classes(c),
            i => panic!("unknown attribute {i}"),
        }
    }

    fn get_text(constant_pool: &ConstantPool, name_index: usize) -> String {
        let pool_info = constant_pool
            .infos
            .get(name_index)
            .expect(&format!("no constant pool entry found for {name_index}"));
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
        let mut exceptions = Vec::new();
        for _ in 0..exception_table_length {
            exceptions.push(Exception::new(c));
        }

        Attribute::Code {
            max_stacks,
            max_locals,
            code,
            attributes: Attribute::attributes(c, constant_pool),
            exceptions,
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

    fn local_variable_table(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        let local_variable_table_length = parse_u16(c) as usize;

        let mut local_variable_table = Vec::with_capacity(local_variable_table_length);
        for _ in 0..local_variable_table_length {
            local_variable_table.push(LocalVariable::new(c));
        }

        Attribute::LocalVariableTable {
            local_variable_table,
        }
    }

    fn stack_map_table(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        let number_of_entries = parse_u16(c);

        let mut entries = Vec::new();
        for _ in 0..number_of_entries {
            entries.push(StackMapFrame::new(c));
        }

        Attribute::StackMapTable { entries }
    }

    fn exceptions(c: &mut Cursor<&Vec<u8>>) -> Self {
        let number_of_exceptions = parse_u16(c);

        let mut exception_index_table = Vec::new();
        for _ in 0..number_of_exceptions {
            exception_index_table.push(parse_u16(c));
        }

        Attribute::Exceptions {
            exception_index_table,
        }
    }

    fn local_variable_type_table(c: &mut Cursor<&Vec<u8>>) -> Self {
        let mut local_variable_type_table = Vec::new();
        for _ in 0..parse_u16(c) {
            local_variable_type_table.push(LocalVariableType::new(c));
        }

        Self::LocalVariableTypeTable {
            local_variable_type_table,
        }
    }

    fn signature(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self::Signature {
            signature_index: parse_u16(c),
        }
    }

    fn nest_members(c: &mut Cursor<&Vec<u8>>) -> Self {
        let number_of_classes = parse_u16(c) as usize;
        let mut classes = Vec::with_capacity(number_of_classes);
        for _ in 0..number_of_classes {
            classes.push(parse_u16(c));
        }

        Self::NestMembers { classes }
    }

    fn bootstrap_methods(c: &mut Cursor<&Vec<u8>>) -> Self {
        let num_bootstrap_methods = parse_u16(c) as usize;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods);
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod::new(c));
        }

        Self::BootstrapMethods { bootstrap_methods }
    }

    fn inner_classes(c: &mut Cursor<&Vec<u8>>) -> Self {
        let number_of_classes = parse_u16(c) as usize;
        let mut classes = Vec::with_capacity(number_of_classes);
        for _ in 0..number_of_classes {
            classes.push(InnerClass::new(c));
        }

        Self::InnerClasses { classes }
    }
}
