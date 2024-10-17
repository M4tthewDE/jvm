use std::io::{Cursor, Seek};

use annotation::Annotation;
use bootstrap_method::BootstrapMethod;
use exception::Exception;
use inner_class::InnerClass;
use line_number_table_entry::LineNumberTableEntry;
use local_variable::{LocalVariable, LocalVariableType};
use stack_map_frame::StackMapFrame;
use tracing::{debug, instrument};

use crate::parser::{parse_u32, parse_vec};

use super::{constant_pool::ConstantPool, parse_u16};

mod annotation;
mod bootstrap_method;
pub mod exception;
mod inner_class;
pub mod line_number_table_entry;
mod local_variable;
mod stack_map_frame;

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

        let text = Attribute::get_text(constant_pool, name_index);
        debug!("parsing attribute {text}");

        match text.as_str() {
            "Code" => Self::code(c, constant_pool),
            "LineNumberTable" => Self::line_number_table(c),
            "SourceFile" => Self::source_file(c),
            "ConstantValue" => Self::constant_value(c),
            "RuntimeVisibleAnnotations" => Self::runtime_visible_annotations(c),
            "LocalVariableTable" => Self::local_variable_table(c),
            "StackMapTable" => Self::stack_map_table(c),
            "Exceptions" => Self::exceptions(c),
            "LocalVariableTypeTable" => Self::local_variable_type_table(c),
            "Signature" => Self::signature(c),
            "Deprecated" => Self::Deprecated,
            "NestMembers" => Self::nest_members(c),
            "BootstrapMethods" => Self::bootstrap_methods(c),
            "InnerClasses" => Self::inner_classes(c),
            i => panic!("unknown attribute {i}"),
        }
    }

    fn get_text(constant_pool: &ConstantPool, name_index: usize) -> String {
        constant_pool
            .utf8(name_index)
            .unwrap_or_else(|| panic!("no constant pool utf8 entry found for {name_index}"))
    }

    pub fn attributes(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Vec<Self> {
        let attributes_count = parse_u16(c) as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(Self::new(c, constant_pool));
        }
        attributes
    }

    fn source_file(c: &mut Cursor<&Vec<u8>>) -> Attribute {
        Attribute::SourceFile {
            source_file_index: parse_u16(c),
        }
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>) -> Self {
        let table_length = parse_u16(c) as usize;
        let mut table = Vec::with_capacity(table_length);
        for _ in 0..table_length {
            table.push(LineNumberTableEntry::new(c));
        }
        Self::LineNumberTable { table }
    }

    fn code(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Self {
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

        Self::Code {
            max_stacks,
            max_locals,
            code,
            attributes: Self::attributes(c, constant_pool),
            exceptions,
        }
    }

    fn constant_value(c: &mut Cursor<&Vec<u8>>) -> Self {
        Self::ConstantValue {
            constant_value_index: parse_u16(c),
        }
    }

    fn runtime_visible_annotations(c: &mut Cursor<&Vec<u8>>) -> Self {
        let num_annotations = parse_u16(c);

        let mut annotations = Vec::new();
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(c));
        }

        Self::RuntimeVisibleAnnotations { annotations }
    }

    fn local_variable_table(c: &mut Cursor<&Vec<u8>>) -> Self {
        let local_variable_table_length = parse_u16(c) as usize;

        let mut local_variable_table = Vec::with_capacity(local_variable_table_length);
        for _ in 0..local_variable_table_length {
            local_variable_table.push(LocalVariable::new(c));
        }

        Self::LocalVariableTable {
            local_variable_table,
        }
    }

    fn stack_map_table(c: &mut Cursor<&Vec<u8>>) -> Self {
        let number_of_entries = parse_u16(c);

        let mut entries = Vec::new();
        for _ in 0..number_of_entries {
            entries.push(StackMapFrame::new(c));
        }

        Self::StackMapTable { entries }
    }

    fn exceptions(c: &mut Cursor<&Vec<u8>>) -> Self {
        let number_of_exceptions = parse_u16(c);

        let mut exception_index_table = Vec::new();
        for _ in 0..number_of_exceptions {
            exception_index_table.push(parse_u16(c));
        }

        Self::Exceptions {
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
