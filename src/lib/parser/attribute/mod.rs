use anyhow::{bail, Context, Result};
use std::io::{Cursor, Seek};

use annotation::Annotation;
use bootstrap_method::BootstrapMethod;
use exception::Exception;
use inner_class::InnerClass;
use line_number_table_entry::LineNumberTableEntry;
use local_variable::{LocalVariable, LocalVariableType};
use stack_map_frame::StackMapFrame;
use tracing::{instrument, trace};

use super::{
    constant_pool::{ConstantPool, Index},
    parse_u16, parse_u32, parse_vec,
};

mod annotation;
mod bootstrap_method;
mod exception;
mod inner_class;
mod line_number_table_entry;
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
        source_file_index: Index,
    },
    ConstantValue {
        constant_value_index: Index,
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
        exception_index_table: Vec<Index>,
    },
    Signature {
        signature_index: Index,
    },
    Deprecated,
    NestMembers {
        classes: Vec<Index>,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },

    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: Index,
        method_index: Index,
    },
    NestHost {
        host_class_index: Index,
    },
}

impl Attribute {
    #[instrument(skip_all, name = "attribute")]
    pub fn new(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Result<Attribute> {
        let name_index = Index::new(parse_u16(c)?);
        c.seek_relative(4)?;

        let text = Attribute::get_text(constant_pool, &name_index)?;
        trace!("parsing attribute {text}");

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
            "Deprecated" => Ok(Self::Deprecated),
            "NestMembers" => Self::nest_members(c),
            "BootstrapMethods" => Self::bootstrap_methods(c),
            "InnerClasses" => Self::inner_classes(c),
            "EnclosingMethod" => Self::enclosing_method(c),
            "NestHost" => Self::nest_host(c),
            i => bail!("unknown attribute {i}"),
        }
    }

    fn get_text(constant_pool: &ConstantPool, name_index: &Index) -> Result<String> {
        constant_pool.utf8(name_index).context(format!(
            "no constant pool utf8 entry found for {name_index:?}"
        ))
    }

    pub fn attributes(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Result<Vec<Self>> {
        let attributes_count = parse_u16(c)? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(Self::new(c, constant_pool)?);
        }

        Ok(attributes)
    }

    fn source_file(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Attribute::SourceFile {
            source_file_index: Index::new(parse_u16(c)?),
        })
    }

    fn line_number_table(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let table_length = parse_u16(c)? as usize;
        let mut table = Vec::with_capacity(table_length);
        for _ in 0..table_length {
            table.push(LineNumberTableEntry::new(c)?);
        }
        Ok(Self::LineNumberTable { table })
    }

    fn code(c: &mut Cursor<&Vec<u8>>, constant_pool: &ConstantPool) -> Result<Self> {
        let max_stacks = parse_u16(c)?;
        let max_locals = parse_u16(c)?;

        let code_length = parse_u32(c)? as usize;
        if code_length == 0 {
            bail!("code is empty");
        }
        let code = parse_vec(c, code_length)?;

        let exception_table_length = parse_u16(c)?;
        let mut exceptions = Vec::new();
        for _ in 0..exception_table_length {
            exceptions.push(Exception::new(c)?);
        }

        Ok(Self::Code {
            max_stacks,
            max_locals,
            code,
            attributes: Self::attributes(c, constant_pool)?,
            exceptions,
        })
    }

    fn constant_value(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Self::ConstantValue {
            constant_value_index: Index::new(parse_u16(c)?),
        })
    }

    fn runtime_visible_annotations(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let num_annotations = parse_u16(c)?;

        let mut annotations = Vec::new();
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(c)?);
        }

        Ok(Self::RuntimeVisibleAnnotations { annotations })
    }

    fn local_variable_table(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let local_variable_table_length = parse_u16(c)? as usize;

        let mut local_variable_table = Vec::with_capacity(local_variable_table_length);
        for _ in 0..local_variable_table_length {
            local_variable_table.push(LocalVariable::new(c)?);
        }

        Ok(Self::LocalVariableTable {
            local_variable_table,
        })
    }

    fn stack_map_table(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let number_of_entries = parse_u16(c)?;

        let mut entries = Vec::new();
        for _ in 0..number_of_entries {
            entries.push(StackMapFrame::new(c)?);
        }

        Ok(Self::StackMapTable { entries })
    }

    fn exceptions(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let number_of_exceptions = parse_u16(c)?;

        let mut exception_index_table = Vec::new();
        for _ in 0..number_of_exceptions {
            exception_index_table.push(Index::new(parse_u16(c)?));
        }

        Ok(Self::Exceptions {
            exception_index_table,
        })
    }

    fn local_variable_type_table(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let mut local_variable_type_table = Vec::new();
        for _ in 0..parse_u16(c)? {
            local_variable_type_table.push(LocalVariableType::new(c)?);
        }

        Ok(Self::LocalVariableTypeTable {
            local_variable_type_table,
        })
    }

    fn signature(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Self::Signature {
            signature_index: Index::new(parse_u16(c)?),
        })
    }

    fn nest_members(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let number_of_classes = parse_u16(c)? as usize;
        let mut classes = Vec::with_capacity(number_of_classes);
        for _ in 0..number_of_classes {
            classes.push(Index::new(parse_u16(c)?));
        }

        Ok(Self::NestMembers { classes })
    }

    fn bootstrap_methods(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let num_bootstrap_methods = parse_u16(c)? as usize;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods);
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod::new(c)?);
        }

        Ok(Self::BootstrapMethods { bootstrap_methods })
    }

    fn inner_classes(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        let number_of_classes = parse_u16(c)? as usize;
        let mut classes = Vec::with_capacity(number_of_classes);
        for _ in 0..number_of_classes {
            classes.push(InnerClass::new(c)?);
        }

        Ok(Self::InnerClasses { classes })
    }

    fn enclosing_method(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Self::EnclosingMethod {
            class_index: Index::new(parse_u16(c)?),
            method_index: Index::new(parse_u16(c)?),
        })
    }

    fn nest_host(c: &mut Cursor<&Vec<u8>>) -> Result<Self> {
        Ok(Self::NestHost {
            host_class_index: Index::new(parse_u16(c)?),
        })
    }
}
