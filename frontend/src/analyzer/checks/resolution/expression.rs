use language::*;
use super::*;

pub fn resolve_expression(
    expr: &mut AstExpression,
    ctx: &mut SemanticContext,
    map: &VariableMap,
) {
    match expr {
        AstExpression::Var { identifier, span } => {
            if let Some(info) = map.get(identifier) {
                *identifier = info.unique_name.clone();
            } else {
                ctx.diagnostics_mut().push(Diagnostic::error(
                    *span,
                    DiagnosticKind::UseOfUndeclaredVariable { name: identifier.clone() },
                ));
            }
        }
        AstExpression::Assignment { left, right, .. } => {
            resolve_expression(left, ctx, map);
            resolve_expression(right, ctx, map);

            if !matches!(**left, AstExpression::Var { .. }) {
                ctx.diagnostics_mut().push(Diagnostic::error(
                    left.get_span(),
                    DiagnosticKind::Custom("invalid lvalue in assignment".to_string()),
                ));
            }
        }
        AstExpression::Unary { operand, .. } => {
            resolve_expression(operand, ctx, map);
        }
        AstExpression::Binary { left, right, .. } => {
            resolve_expression(left, ctx, map);
            resolve_expression(right, ctx, map);
        }
        AstExpression::Conditional { condition, then_branch, else_branch, .. } => {
            resolve_expression(condition, ctx, map);
            resolve_expression(then_branch, ctx, map);
            resolve_expression(else_branch, ctx, map);
        }
        AstExpression::Constant { .. } => {}
    }
}
