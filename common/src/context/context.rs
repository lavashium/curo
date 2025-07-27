use crate::*;
use language::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct CompilerContext<'ctx> {
    pub diagnostics: &'ctx mut DiagnosticManager,
    pub tempgen: &'ctx mut TempGen,

    pub symtable: SymbolTable
}