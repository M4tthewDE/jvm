use super::{class::Class, stack::Word};

#[derive(Debug, Clone)]
pub struct Instance {
    _class: Class,
    _instance_variables: Vec<Word>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        assert_eq!(
            class.fields.len(),
            0,
            "instance fields are not supported yet"
        );
        Self {
            _class: class,
            _instance_variables: Vec::new(),
        }
    }
}
