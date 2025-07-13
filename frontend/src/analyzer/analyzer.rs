use super::checks::*;
use language::*;
use common::*;
use constructors::constructors;
use accessors::accessors;
use zawarudo::zawarudo;

#[accessors]
pub struct SemanticContext<'a> {
    pub diagnostics: &'a mut DiagnosticsManager,
    pub temp_gen: &'a mut TempGen,
    pub scopes: Vec<IdentifierMap>,
    pub loop_depth: usize,
    pub inside_function: bool
}

impl<'a> SemanticContext<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticsManager, temp_gen: &'a mut TempGen) -> Self {
        Self { 
            diagnostics, 
            temp_gen,
            scopes: vec![],
            loop_depth: 0,
            inside_function: false,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(IdentifierMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn lookup_identifier(&self, identifier: &str) -> Option<&IdentifierInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(identifier) {
                return Some(info);
            }
        }
        None
    }
}

#[accessors]
#[constructors]
pub struct Analyzer<'a> {
    program: &'a mut AstProgram,
    ctx: SemanticContext<'a>,
}

impl<'a> Analyzer<'a> {
    #[zawarudo(label = "Semantic Analyzer")]
    pub fn analyze(&mut self) {
        CHECKS::run_all(self.program, &mut self.ctx);
    }
} 