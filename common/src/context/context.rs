use crate::*;
use language::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct CompilerContext<'ctx> {
    pub diagnostics: &'ctx mut DiagnosticsManager,
    pub tempgen: &'ctx mut TempGen,
}