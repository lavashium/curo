use accessors::accessors;
use constructors::constructors;
use std::collections::HashMap;
use language::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitialValue {
    Tentative,
    Initial(String),
    NoInitializer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierAttrs {
    FunAttr {
        defined: bool,
        global: bool,
        stack_frame_size: usize,
    },
    StaticAttr {
        init: InitialValue,
        global: bool,
    },
    LocalAttr,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub ty: AstType,
    pub attrs: IdentifierAttrs,
}

#[accessors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    pub fn add_automatic_var(&mut self, name: String, ty: AstType) {
        self.table.insert(
            name, 
            Symbol {
                ty,
                attrs: IdentifierAttrs::LocalAttr,
            }
        );
    }

    pub fn add_static_var(&mut self, name: String, ty: AstType, global: bool, init: InitialValue) {
        self.table.insert(
            name, 
            Symbol {
                ty,
                attrs: IdentifierAttrs::StaticAttr {
                    init, 
                    global 
                },
            }
        );
    }

    pub fn add_fun(&mut self, name: String, ty: AstType, global: bool, defined: bool) {
        self.table.insert(
            name, 
            Symbol {
                ty,
                attrs: IdentifierAttrs::FunAttr {
                    global,
                    defined,
                    stack_frame_size: 0,
                },
            }
        );
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.table.get_mut(name)
    }

    pub fn is_global(&self, name: &str) -> bool {
        self.get(name).map_or(false, |sym| match &sym.attrs {
            IdentifierAttrs::LocalAttr => false,
            IdentifierAttrs::StaticAttr { global, .. } => *global,
            IdentifierAttrs::FunAttr { global, .. } => *global,
        })
    }

    pub fn is_static(&self, name: &str) -> bool {
        match self.get(name) {
            Some(Symbol { attrs: IdentifierAttrs::StaticAttr { .. }, .. }) => true,
            Some(Symbol { attrs: IdentifierAttrs::FunAttr { .. }, .. }) => {
                panic!("Internal error: functions don't have storage duration")
            }
            _ => false,
        }
    }

    pub fn bindings(&self) -> Vec<(&String, &Symbol)> {
        self.table.iter().collect()
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }

    pub fn set_bytes_required(&mut self, name: &str, bytes_required: usize) {
        match self.table.get_mut(name) {
            Some(Symbol {
                attrs: IdentifierAttrs::FunAttr {
                    stack_frame_size, ..
                },
                ..
            }) => {
                *stack_frame_size = bytes_required;
            }
            _ => panic!("Internal error: not a function"),
        }
    }

    pub fn get_bytes_required(&self, name: &str) -> usize {
        match self.table.get(name) {
            Some(Symbol {
                attrs: IdentifierAttrs::FunAttr {
                    stack_frame_size, ..
                },
                ..
            }) => *stack_frame_size,
            _ => panic!("Internal error: not a function"),
        }
    }
}
