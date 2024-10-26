use anyhow::{bail, Result};
use std::fmt::Display;

use super::{class::Class, stack::Word};

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    _class: Class,
    _instance_variables: Vec<Word>,
}

impl Instance {
    pub fn new(class: Class) -> Result<Self> {
        if class.fields.len() != 0 {
            bail!("instance fields are not supported yet");
        }

        Ok(Self {
            _class: class,
            _instance_variables: Vec::new(),
        })
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instance of {}", self._class)
    }
}
