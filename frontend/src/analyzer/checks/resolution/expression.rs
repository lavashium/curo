use language::*;
use super::*;


pub fn resolve_expression(
    expr: &mut AstExpression,
    ctx: &mut SemanticContext,
    map: &IdentifierMap,
) {
    match expr {
        AstExpression::Var { identifier, span } => {
            if let Some(info) = map.get(identifier) {
                *identifier = info.unique_name.clone();
            } else {
                ctx.diagnostics.push(Diagnostic::error(
                    *span,
                    DiagnosticKind::UseOfUndeclared {
                        name: identifier.clone(),
                    },
                ));
            }
        }
        AstExpression::Constant { .. } => (),
        AstExpression::Unary { operand, .. } => {
            resolve_expression(operand, ctx, map);
        }
        AstExpression::Binary { operator, left, right, span } => {
            resolve_expression(left,  ctx, map);
            resolve_expression(right, ctx, map);

            let is_cmp = matches!(operator,
                AstBinaryKind::LessThan
                | AstBinaryKind::LessOrEqual 
                | AstBinaryKind::GreaterThan 
                | AstBinaryKind::GreaterOrEqual 
                | AstBinaryKind::Equal 
                | AstBinaryKind::NotEqual
            );
            if is_cmp && matches!(**right, AstExpression::Assignment { .. }) {
                push_error(ctx, *span, DiagnosticKind::Custom("mixed precedence assignment".into()));
            }
        }

        AstExpression::Assignment { left, right, span } => {
            resolve_expression(left,  ctx, map);
            resolve_expression(right, ctx, map);

            if !matches!(**left, AstExpression::Var { .. }) {
                push_error(ctx, *span, DiagnosticKind::Custom("invalid lvalue".into()));
            }
        }
        AstExpression::Conditional {
            condition,
            then_branch,
            else_branch,
            span,
        } => {
            resolve_expression(condition, ctx, map);
            resolve_expression(then_branch, ctx, map);
            resolve_expression(else_branch, ctx, map);

        }
        AstExpression::FunctionCall { identifier, args, span } => {
            if let Some(info) = map.get(identifier) {
                *identifier = info.unique_name.clone();
            } else {
                push_error(
                    ctx,
                    *span,
                    DiagnosticKind::UseOfUndeclared {
                        name: identifier.clone(),
                    },
                );
            }

            for arg in args {
                resolve_expression(arg, ctx, map);
            }
        }
    }
}