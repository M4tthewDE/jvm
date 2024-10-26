use anyhow::{Context, Result};
use std::collections::HashMap;

use anyhow::bail;
use class::Class;
use code::Code;
use field::Field;
use loader::ClassLoader;
use method::Method;
use stack::{Stack, Word};
use tracing::{debug, info};

use crate::{
    parser::constant_pool::{Index, NameAndType},
    ClassIdentifier,
};

mod class;
mod code;
mod field;
mod instance;
pub mod loader;
mod method;
mod native;
mod op;
mod stack;

pub struct Executor {
    class_loader: ClassLoader,
    initialized_classes: HashMap<ClassIdentifier, Class>,
    class_being_initialized: Option<Class>,
    stack: Stack,
}

impl Executor {
    pub fn new(class_loader: ClassLoader) -> Self {
        Self {
            class_loader,
            initialized_classes: HashMap::new(),
            class_being_initialized: None,
            stack: Stack::new(),
        }
    }

    pub fn execute(&mut self, class_identifier: ClassIdentifier) -> Result<()> {
        let class = self.class_loader.load(class_identifier)?;
        self.execute_main_method(class)
    }

    fn get_class(&self, class_identifier: &ClassIdentifier) -> Option<Class> {
        self.initialized_classes.get(class_identifier).cloned()
    }

    fn execute_main_method(&mut self, class: Class) -> Result<()> {
        self.initialize_class(class.clone())?;
        let class = self
            .get_class(&class.identifier)
            .context(format!("class not found {}", &class.identifier))?;
        let method = class.main_method()?;

        // TODO: add []String args, see invokestatic for reference
        let code = Code::new(method.code_attribute()?);
        self.stack.create(class, method, code, vec![]);
        self.execute_code()
    }

    fn execute_clinit(&mut self, class: Class, method: Method) -> Result<()> {
        info!("Executing clinit for {}", class.identifier);
        let code = Code::new(method.code_attribute()?);
        self.stack.create(class, method, code, vec![]);
        self.execute_code()?;
        todo!("after execute clinit");
    }

    fn initialize_class(&mut self, class: Class) -> Result<()> {
        if self.initialized_classes.contains_key(&class.identifier) {
            return Ok(());
        }

        if let Some(class_being_initialized) = &self.class_being_initialized {
            if class_being_initialized.identifier == class.identifier {
                return Ok(());
            }
        }

        self.class_being_initialized = Some(class.clone());

        if let Some(clinit) = class.clinit_method() {
            self.execute_clinit(class.clone(), clinit)?;
        }
        if let Some(ci) = &self.class_being_initialized {
            self.initialized_classes
                .insert(class.identifier.clone(), ci.clone());
            self.class_being_initialized = None;
            Ok(())
        } else {
            bail!("no class is being initialized")
        }
    }

    fn execute_code(&mut self) -> Result<()> {
        loop {
            let op_code = self.stack.get_opcode()?;
            let op = op::get_op(op_code)?;
            info!("Executing {}", op::name(op_code)?);
            op(self)?;
        }
    }

    fn invoke_static(
        &mut self,
        class_identifier: ClassIdentifier,
        name_and_type: NameAndType,
    ) -> Result<()> {
        debug!("Invoking {name_and_type} in {class_identifier}");
        let class = self.class_loader.load(class_identifier.clone())?;
        self.initialize_class(class.clone())?;
        let method_descriptor = &name_and_type.descriptor.method_descriptor()?;

        let operands = self
            .stack
            .pop_operands(method_descriptor.parameters.len())?;
        if class.is_native(&name_and_type.name, method_descriptor)? {
            if let Some(word) = native::invoke_static(
                self,
                class.identifier,
                name_and_type.name,
                method_descriptor.parameters.clone(),
                operands,
            )? {
                self.stack.push_operand(word);
            }

            Ok(())
        } else {
            let method = class.method(&name_and_type.name, method_descriptor)?;
            let code = Code::new(method.code_attribute()?);
            self.stack.create(class, method, code, operands);
            self.execute_code()?;
            bail!("after invoke_static has executed its code");
        }
    }

    fn resolve_field(&mut self, field_index: &Index) -> Result<Field> {
        let (class_identifier, name_and_type) = self.stack.lookup_field(field_index)?;
        let class = self.resolve_class(class_identifier.clone())?;
        let field = class.field(&name_and_type)?;
        self.initialize_class(class)?;
        Ok(field)
    }

    fn resolve_class(&mut self, identifier: ClassIdentifier) -> Result<Class> {
        let class = self.class_loader.load(identifier)?;
        if !self.stack.can_access(&class)? {
            bail!("{:?} is not allowed to access {class}, we should throw IllegalAccessError once we support exceptions", self.stack);
        }

        Ok(class)
    }

    fn assign_static_field(&mut self, field: &Field, value: &Word) -> Result<()> {
        if let Some(ref mut class) = self.class_being_initialized {
            debug!("Assigning {field} in {class}");
            class.set_field(field, value);
            self.class_being_initialized = Some(class.clone());
            Ok(())
        } else {
            bail!("no class is being initialized")
        }
    }

    fn pc(&mut self, n: usize) -> Result<()> {
        self.stack.pc(n)
    }
}
