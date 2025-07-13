mod program;
mod function_declaration;
mod variable_declaration;
mod block;
mod block_item;
mod statement;
mod expression;
mod declaration;

use program::*;
use function_declaration::*;
use variable_declaration::*;
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

#[derive(Clone)]
pub enum IdentifierKind {
    Variable,
    Function,
}

#[constructors]
#[accessors]
#[derive(Clone)]
pub struct IdentifierInfo {
    pub unique_name: String,
    pub has_linkage: bool,
    pub from_current_scope: bool,
}

pub type IdentifierMap = HashMap<String, IdentifierInfo>;

pub struct VariableResolutionCheck;

impl SemanticCheck for VariableResolutionCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        let mut map = IdentifierMap::new();
        resolve_program(ast, ctx, &mut map)
    }
}

pub fn copy_identifier_map(src: &IdentifierMap) -> IdentifierMap {
    src.iter()
        .map(|(k, v)| {
            (
                k.clone(),
                IdentifierInfo {
                    unique_name: v.unique_name.clone(),
                    from_current_scope: false,
                    has_linkage: v.has_linkage,
                },
            )
        })
        .collect()
}

pub fn is_lvalue(expr: &AstExpression) -> bool {
    matches!(expr,
        AstExpression::Var { .. }
    )
}

pub fn push_error(ctx: &mut SemanticContext<'_>, span: Span, kind: DiagnosticKind) {
    ctx.diagnostics
        .push(Diagnostic::error(span, kind));
}

pub fn ensure_seen(expr_span: Span, ident: &str, ctx: &mut SemanticContext<'_>, map: &IdentifierMap) {
    if !map.contains_key(ident) {
        push_error(
            ctx,
            expr_span,
            DiagnosticKind::UseOfUndeclared {
                name: ident.to_string(),
            },
        );
    }
}