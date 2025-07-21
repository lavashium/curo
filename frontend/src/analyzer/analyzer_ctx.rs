use std::collections::HashMap;
use common::*;
use language::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct AnalyzerContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub scopes: Vec<IdentifierMap>,
    pub loop_depth: usize,
    pub inside_function: bool,
}

pub type IdentifierMap = HashMap<String, IdentifierInfo>;

#[derive(Clone)]
pub struct IdentifierInfo {
    pub unique_name: String,
    pub has_linkage: bool,
    pub from_current_scope: bool,
}

impl<'scp, 'ctx> AnalyzerContext<'scp, 'ctx> {
    pub fn push_scope(&mut self, inherit: bool) {
        let new_scope = if inherit && !self.scopes.is_empty() {
            self.scopes.last().unwrap().iter().map(|(k, v)| {
                (k.clone(), IdentifierInfo {
                    unique_name: v.unique_name.clone(),
                    has_linkage: v.has_linkage,
                    from_current_scope: false,
                })
            }).collect()
        } else {
            IdentifierMap::new()
        };
        self.scopes.push(new_scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("No scope to pop");
    }

    pub fn current_scope(&self) -> &IdentifierMap {
        self.scopes.last().expect("No current scope")
    }

    pub fn current_scope_mut(&mut self) -> &mut IdentifierMap {
        self.scopes.last_mut().expect("No current scope")
    }

    pub fn declare_identifier(&mut self, name: &str, has_linkage: bool, span: Span) -> Option<String> {
        let unique_name = self.ctx.tempgen.temp_from(name.to_string());
        let scope = self.current_scope_mut();
        
        if let Some(existing) = scope.get(name) {
            if existing.from_current_scope && !(has_linkage && existing.has_linkage) {
                self.ctx.diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::DuplicateDeclaration {
                        name: name.to_string(),
                    },
                ));
                return None;
            }
        }

        scope.insert(
            name.to_string(),
            IdentifierInfo {
                unique_name: unique_name.clone(),
                has_linkage,
                from_current_scope: true,
            },
        );
        Some(unique_name)
    }
}