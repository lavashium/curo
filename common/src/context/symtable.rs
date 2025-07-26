use accessors::accessors;
use constructors::constructors;
use std::collections::HashMap;
use language::*;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub ty: AstType,
    pub defined: bool,
    pub stack_frame_size: usize,
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

    pub fn add_var(&mut self, name: String, ty: AstType) {
        self.table.insert(
            name, 
            Symbol::new(ty, false, 0)
        );
    }

    pub fn add_fun(&mut self, name: String, ty: AstType, is_defined: bool) {
        self.table.insert(
            name, 
            Symbol::new(ty, is_defined, 0)
        );
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name)
    }
}