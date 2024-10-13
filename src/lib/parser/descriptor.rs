#[derive(Clone, Debug, PartialEq, Eq)]
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
    Array(Box<FieldType>),
}

impl FieldType {
    fn new(text: &str) -> FieldType {
        match text.chars().next().unwrap() {
            'B' => FieldType::Byte,
            'C' => FieldType::Char,
            'D' => FieldType::Double,
            'F' => FieldType::Float,
            'I' => FieldType::Int,
            'J' => FieldType::Long,
            'L' => FieldType::Class(text[1..text.find(';').unwrap()].to_string()),
            'S' => FieldType::Short,
            'Z' => FieldType::Boolean,
            '[' => FieldType::Array(Box::new(FieldType::new(&text[1..]))),
            c => panic!("INVALID: {c}"),
        }
    }

    fn size(&self) -> usize {
        match self {
            FieldType::Byte => "B".len(),
            FieldType::Char => "C".len(),
            FieldType::Double => "D".len(),
            FieldType::Float => "F".len(),
            FieldType::Int => "I".len(),
            FieldType::Long => "J".len(),
            FieldType::Class(name) => "L".len() + name.len() + ";".len(),
            FieldType::Short => "S".len(),
            FieldType::Boolean => "B".len(),
            FieldType::Array(field_type) => "[".len() + field_type.size(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReturnDescriptor {
    Type(FieldType),
    Void,
}

impl ReturnDescriptor {
    fn new(text: &str) -> ReturnDescriptor {
        if text == "V" {
            ReturnDescriptor::Void
        } else {
            ReturnDescriptor::Type(FieldType::new(text))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodDescriptor {
    parameters: Vec<FieldType>,
    return_descriptor: ReturnDescriptor,
}

impl MethodDescriptor {
    fn new(text: &str) -> Self {
        let parameters = Self::parameters(&text[1..text.find(')').unwrap()]);
        MethodDescriptor {
            parameters,
            return_descriptor: ReturnDescriptor::new(&text[text.find(')').unwrap() + 1..]),
        }
    }

    fn parameters(text: &str) -> Vec<FieldType> {
        let mut parameters = Vec::new();

        let mut i = 0;
        loop {
            let parameter = FieldType::new(&text[i..]);
            i += parameter.size();
            parameters.push(parameter);

            if i == text.len() {
                break;
            }
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
