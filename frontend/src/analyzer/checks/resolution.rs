use std::collections::HashMap;
use super::*;
use language::*;
use common::*;

pub struct VariableResolutionCheck;

impl SemanticCheck for VariableResolutionCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        let mut variable_map = HashMap::new();
        let body = ast.function_definition_mut().body_mut();

        for item in body.iter_mut() {
            match item {
                AstBlockItem::Declaration(decl) => {
                    *decl = resolve_declaration(decl.clone(), ctx, &mut variable_map);
                }
                AstBlockItem::Statement(stmt) => {
                    *stmt = resolve_statement(stmt.clone(), ctx, &variable_map);
                }
            }
        }
    }
}

fn resolve_declaration(
    decl: AstDeclaration,
    ctx: &mut SemanticContext,
    variable_map: &mut HashMap<String, String>,
) -> AstDeclaration {
    let name = decl.name().clone();

    if variable_map.contains_key(&name) {
        ctx.diagnostics_mut().push(Diagnostic::error(
            decl.get_span(),
            DiagnosticKind::DuplicateVariableDeclaration { name: name.clone() },
        ));
    }

    let unique_name = ctx.temp_gen_mut().temp_from(name.clone());
    variable_map.insert(name, unique_name.clone());

    let new_init = decl
        .init()
        .as_ref()
        .map(|e| resolve_expression(e.clone(), ctx, variable_map));

    AstDeclaration::new(unique_name, new_init, decl.get_span())
}

fn resolve_statement(
    stmt: AstStatement,
    ctx: &mut SemanticContext,
    variable_map: &HashMap<String, String>,
) -> AstStatement {
    match stmt {
        AstStatement::Return { expression } => AstStatement::Return {
            expression: resolve_expression(expression, ctx, variable_map),
        },
        AstStatement::Expression { expression } => AstStatement::Expression {
            expression: resolve_expression(expression, ctx, variable_map),
        },
        AstStatement::If { condition, then_branch, else_branch } => AstStatement::If {
            condition: resolve_expression(condition, ctx, variable_map),
            then_branch: Box::new(resolve_statement(*then_branch, ctx, variable_map)),
            else_branch: else_branch.map(|branch| Box::new(resolve_statement(*branch, ctx, variable_map))),
        },
        AstStatement::Null => AstStatement::Null,
        
    }
}

fn resolve_expression(
    expr: AstExpression,
    ctx: &mut SemanticContext,
    variable_map: &HashMap<String, String>,
) -> AstExpression {
    match expr {
        AstExpression::Var { identifier, span } => {
            if let Some(unique) = variable_map.get(&identifier) {
                AstExpression::Var { identifier: unique.clone(), span }
            } else {
                ctx.diagnostics_mut().push(Diagnostic::error(
                    span,
                    DiagnosticKind::UseOfUndeclaredVariable { name: identifier.clone() },
                ));
                AstExpression::Var { identifier, span }
            }
        }
        AstExpression::Assignment { left, right, span } => {
            let resolved_left = resolve_expression(*left, ctx, variable_map);
            if let AstExpression::Var { .. } = resolved_left {
                let resolved_right = resolve_expression(*right, ctx, variable_map);
                AstExpression::Assignment {
                    left: Box::new(resolved_left),
                    right: Box::new(resolved_right),
                    span,
                }
            } else {
                ctx.diagnostics_mut().push(Diagnostic::error(
                    resolved_left.get_span(),
                    DiagnosticKind::Custom("invalid lvalue in assignment".to_string()),
                ));
                let resolved_right = resolve_expression(*right, ctx, variable_map);
                AstExpression::Assignment {
                    left: Box::new(resolved_left),
                    right: Box::new(resolved_right),
                    span,
                }
            }
        }
        AstExpression::Unary { operator, operand, span } => AstExpression::Unary {
            operator,
            operand: Box::new(resolve_expression(*operand, ctx, variable_map)),
            span,
        },
        AstExpression::Binary { operator, left, right, span } => AstExpression::Binary {
            operator,
            left: Box::new(resolve_expression(*left, ctx, variable_map)),
            right: Box::new(resolve_expression(*right, ctx, variable_map)),
            span,
        },
        AstExpression::Constant { .. } => expr,
        AstExpression::Conditional { condition, then_branch, else_branch, span } => AstExpression::Conditional {
            condition: Box::new(resolve_expression(*condition, ctx, variable_map)),
            then_branch: Box::new(resolve_expression(*then_branch, ctx, variable_map)),
            else_branch: Box::new(resolve_expression(*else_branch, ctx, variable_map)),
            span,
        },
    }
}
