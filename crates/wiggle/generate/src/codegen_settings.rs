use crate::config::{AsyncConf, ErrorConf};
use anyhow::{anyhow, Error};
use std::rc::Rc;
use witx::{Document, Id, InterfaceFunc, Module, NamedType, TypeRef};

pub use crate::config::Asyncness;

pub struct CodegenSettings {
    pub errors: ErrorTransform,
    pub async_: AsyncConf,
    pub wasmtime: bool,
    // Disabling this feature makes it possible to remove all of the tracing
    // code emitted in the Wiggle-generated code; this can be helpful while
    // inspecting the code (e.g., with `cargo expand`).
    pub tracing: bool,
}
impl CodegenSettings {
    pub fn new(
        error_conf: &ErrorConf,
        async_: &AsyncConf,
        doc: &Document,
        wasmtime: bool,
        tracing: bool,
    ) -> Result<Self, Error> {
        let errors = ErrorTransform::new(error_conf, doc)?;
        Ok(Self {
            errors,
            async_: async_.clone(),
            wasmtime,
            tracing,
        })
    }
    pub fn get_async(&self, module: &Module, func: &InterfaceFunc) -> Asyncness {
        self.async_.get(module.name.as_str(), func.name.as_str())
    }
}

pub struct ErrorTransform {
    types: Vec<Rc<NamedType>>,
}

impl ErrorTransform {
    pub fn empty() -> Self {
        Self { types: Vec::new() }
    }
    pub fn new(conf: &ErrorConf, doc: &Document) -> Result<Self, Error> {
        let types = conf
            .iter()
            .map(|ident| {
                doc.typename(&Id::new(ident.to_string()))
                    .ok_or_else(|| anyhow!("No witx typename \"{}\" found", ident.to_string()))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Self { types })
    }

    pub fn iter(&self) -> impl Iterator<Item = &NamedType> {
        self.types.iter().map(|r| std::borrow::Borrow::borrow(r))
    }

    pub fn for_abi_error(&self, tref: &TypeRef) -> bool {
        match tref {
            TypeRef::Name(nt) => self.for_name(nt),
            TypeRef::Value { .. } => false,
        }
    }

    pub fn for_name(&self, nt: &NamedType) -> bool {
        self.iter().find(|u| u.name == nt.name).is_some()
    }
}
