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

impl FieldType {
    fn new(text: &str) -> Self {
        match text.chars().next().unwrap() {
            'B' => Self::Byte,
            'C' => Self::Char,
            'D' => Self::Double,
            'F' => Self::Float,
            'I' => Self::Int,
            'J' => Self::Long,
            'L' => Self::Class(Self::class_text(text)),
            'S' => Self::Short,
            'Z' => Self::Boolean,
            '[' => Self::Array(Box::new(Self::new(&text[1..]))),
            c => panic!("INVALID: {c}"),
        }
    }

    fn class_text(text: &str) -> String {
        text[1..text.find(';').unwrap()].to_string()
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

impl ReturnDescriptor {
    fn new(text: &str) -> Self {
        if text == "V" {
            Self::Void
        } else {
            Self::Type(FieldType::new(text))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub parameters: Vec<FieldType>,
    pub return_descriptor: ReturnDescriptor,
}

impl MethodDescriptor {
    pub fn new(text: &str) -> Self {
        Self {
            parameters: Self::parameters(Self::param_text(text)),
            return_descriptor: ReturnDescriptor::new(Self::return_descriptor_text(text)),
        }
    }

    fn param_text(text: &str) -> &str {
        &text[1..text.find(')').unwrap()]
    }

    fn return_descriptor_text(text: &str) -> &str {
        &text[text.find(')').unwrap() + 1..]
    }

    fn parameters(text: &str) -> Vec<FieldType> {
        let mut parameters = Vec::new();

        let mut i = 0;
        while i != text.len() {
            let parameter = FieldType::new(&text[i..]);
            i += parameter.size();
            parameters.push(parameter);
        }

        parameters
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::descriptor::{FieldType, ReturnDescriptor};

    use super::MethodDescriptor;

    #[test]
    fn test_params() {
        let descriptor = MethodDescriptor::new("([Ljava/lang/String;)V");
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
        let descriptor = MethodDescriptor::new("(IZ)Ljava/lang/String;");
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
