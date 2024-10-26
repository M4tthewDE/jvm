use anyhow::{bail, Result};
use std::fmt::Display;

use anyhow::Context;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Descriptor {
    Field(FieldType),
    Method(MethodDescriptor),
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Descriptor::Field(field_type) => write!(f, "{field_type}"),
            Descriptor::Method(d) => write!(f, "{d}"),
        }
    }
}

impl Descriptor {
    pub fn method_descriptor(&self) -> Option<MethodDescriptor> {
        match self {
            Descriptor::Field(_) => None,
            Descriptor::Method(md) => Some(md.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(String),
    Short,
    Boolean,
    Array(Box<Self>),
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Byte => write!(f, "B"),
            FieldType::Char => write!(f, "C"),
            FieldType::Double => write!(f, "D"),
            FieldType::Float => write!(f, "F"),
            FieldType::Int => write!(f, "I"),
            FieldType::Long => write!(f, "J"),
            FieldType::Class(text) => write!(f, "L{text};"),
            FieldType::Short => write!(f, "S"),
            FieldType::Boolean => write!(f, "Z"),
            FieldType::Array(field_type) => write!(f, "[{field_type}"),
        }
    }
}

impl FieldType {
    pub fn new(text: &str) -> Result<Self> {
        Ok(
            match text
                .chars()
                .next()
                .context(format!("no char found in {text}"))?
            {
                'B' => Self::Byte,
                'C' => Self::Char,
                'D' => Self::Double,
                'F' => Self::Float,
                'I' => Self::Int,
                'J' => Self::Long,
                'L' => Self::Class(Self::class_text(text)?),
                'S' => Self::Short,
                'Z' => Self::Boolean,
                '[' => Self::Array(Box::new(Self::new(&text[1..])?)),
                c => bail!("INVALID: {c}"),
            },
        )
    }

    fn class_text(text: &str) -> Result<String> {
        Ok(text[1..text.find(';').context(format!("no : in {text}"))?].to_string())
    }

    fn size(&self) -> usize {
        match self {
            Self::Byte => "B".len(),
            Self::Char => "C".len(),
            Self::Double => "D".len(),
            Self::Float => "F".len(),
            Self::Int => "I".len(),
            Self::Long => "J".len(),
            Self::Class(name) => "L".len() + name.len() + ";".len(),
            Self::Short => "S".len(),
            Self::Boolean => "B".len(),
            Self::Array(field_type) => "[".len() + field_type.size(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReturnDescriptor {
    Type(FieldType),
    Void,
}
impl Display for ReturnDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnDescriptor::Type(field_type) => write!(f, "{field_type}"),
            ReturnDescriptor::Void => write!(f, "V"),
        }
    }
}

impl ReturnDescriptor {
    fn new(text: &str) -> Result<Self> {
        Ok(if text == "V" {
            Self::Void
        } else {
            Self::Type(FieldType::new(text)?)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub parameters: Vec<FieldType>,
    pub return_descriptor: ReturnDescriptor,
}

impl Display for MethodDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = String::new();
        for param in &self.parameters {
            params.push_str(&format!("{param}"));
        }

        write!(f, "({params}){}", self.return_descriptor)
    }
}

impl MethodDescriptor {
    pub fn new(text: &str) -> Result<Self> {
        Ok(Self {
            parameters: Self::parameters(Self::param_text(text)?)?,
            return_descriptor: ReturnDescriptor::new(Self::return_descriptor_text(text)?)?,
        })
    }

    fn param_text(text: &str) -> Result<&str> {
        Ok(&text[1..text.find(')').context(format!("no ) in {text}"))?])
    }

    fn return_descriptor_text(text: &str) -> Result<&str> {
        Ok(&text[text.find(')').context(format!("no ) in {text}"))? + 1..])
    }

    fn parameters(text: &str) -> Result<Vec<FieldType>> {
        let mut parameters = Vec::new();

        let mut i = 0;
        while i != text.len() {
            let parameter = FieldType::new(&text[i..])?;
            i += parameter.size();
            parameters.push(parameter);
        }

        Ok(parameters)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::descriptor::{FieldType, ReturnDescriptor};

    use super::MethodDescriptor;

    #[test]
    fn test_params() {
        let descriptor = MethodDescriptor::new("([Ljava/lang/String;)V").unwrap();
        assert_eq!(
            descriptor,
            MethodDescriptor {
                parameters: vec![FieldType::Array(Box::new(FieldType::Class(
                    "java/lang/String".to_string()
                )))],
                return_descriptor: ReturnDescriptor::Void
            }
        );
    }

    #[test]
    fn test_method_descriptor_multiple_parameters() {
        let descriptor = MethodDescriptor::new("(IZ)Ljava/lang/String;").unwrap();
        assert_eq!(
            descriptor,
            MethodDescriptor {
                parameters: vec![FieldType::Int, FieldType::Boolean,],
                return_descriptor: ReturnDescriptor::Type(FieldType::Class(
                    "java/lang/String".to_string()
                ))
            }
        );
    }
}
