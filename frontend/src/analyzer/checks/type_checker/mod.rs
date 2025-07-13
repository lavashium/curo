mod program;
mod function_declaration;
mod block;
mod block_item;
mod statement;
mod expression;
mod declaration;

use program::*;
use function_declaration::*;
use block::*;
use block_item::*;
use statement::*;
use expression::*;
use declaration::*;

use std::collections::HashMap;
use common::*;
use language::*;
use super::*;
use constructors::constructors;
use accessors::accessors;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    FunType(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub ty: Type,
    pub defined: bool,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable { scopes: Vec::new() };
        table.push_scope();
        table
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("No scope to pop");
    }

    pub fn add_variable(&mut self, name: &str, span: Span, ctx: &mut SemanticContext<'_>) {
        if let Some(current_scope) = self.scopes.last_mut() {
            if current_scope.contains_key(name) {
                push_error(
                    ctx,
                    span,
                    DiagnosticKind::DuplicateDeclaration { name: name.to_string() },
                );
                return;
            }
            current_scope.insert(
                name.to_string(),
                Symbol { ty: Type::Int, defined: true },
            );
        }
    }

    pub fn add_function_decl(
        &mut self,
        name: &str,
        param_count: usize,
        has_body: bool,
        span: Span,
        ctx: &mut SemanticContext<'_>,
    ) {
        let fun_ty = Type::FunType(param_count);

        for scope in &self.scopes {
            if let Some(existing) = scope.get(name) {
                if let Type::FunType(existing_params) = existing.ty {
                    if existing_params != param_count {
                        push_error(
                            ctx,
                            span,
                            DiagnosticKind::Custom(format!(
                                "Conflicting types for '{}'",
                                name
                            )),
                        );
                        return;
                    }
                }
            }
        }

        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(
                name.to_string(),
                Symbol {
                    ty: fun_ty,
                    defined: has_body,
                },
            );
        }
    }


    pub fn get(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }
}

pub struct TypeCheck;

impl SemanticCheck for TypeCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        let mut symbols = SymbolTable::new();
        typecheck_program(ast, &mut symbols, ctx);
    }
}