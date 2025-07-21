use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_expression(expr: &mut TypedExpression, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedExpression, AnalyzerContext<'_, '_>>>::run(expr, ctx)
    }
}

impl Factory<(), TypedExpression, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(expr: &mut TypedExpression, ctx: &mut AnalyzerContext) {
        match expr {
            TypedExpression::Var { identifier, span, .. } => {
                if let Some(info) = ctx.current_scope().get(identifier) {
                    *identifier = info.unique_name.clone();
                } else {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::UseOfUndeclared {
                            name: identifier.clone(),
                        },
                    ));
                }
            }
            TypedExpression::Constant { .. } => {}
            TypedExpression::Unary { operand, .. } => {
                Self::resolve_expression(operand, ctx);
            }
            TypedExpression::Binary { operator, left, right, span, .. } => {
                Self::resolve_expression(left, ctx);
                Self::resolve_expression(right, ctx);

                let is_comparison = matches!(
                    operator,
                    AstBinaryKind::LessThan
                    | AstBinaryKind::LessOrEqual
                    | AstBinaryKind::GreaterThan
                    | AstBinaryKind::GreaterOrEqual
                    | AstBinaryKind::Equal
                    | AstBinaryKind::NotEqual
                );
                
                if is_comparison && matches!(**right, TypedExpression::Assignment { .. }) {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::Custom("mixed precedence assignment".into()),
                    ));
                }
            }
            TypedExpression::Assignment { left, right, span, ..} => {
                Self::resolve_expression(left, ctx);
                Self::resolve_expression(right, ctx);

                if !matches!(**left, TypedExpression::Var { .. }) {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::Custom("invalid lvalue".into()),
                    ));
                }
            }
            TypedExpression::Conditional { condition, then_branch, else_branch, .. } => {
                Self::resolve_expression(condition, ctx);
                Self::resolve_expression(then_branch, ctx);
                Self::resolve_expression(else_branch, ctx);
            }
            TypedExpression::FunctionCall { identifier, args, span, ..} => {
                if let Some(info) = ctx.current_scope().get(identifier) {
                    *identifier = info.unique_name.clone();
                } else {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::UseOfUndeclared {
                            name: identifier.clone(),
                        },
                    ));
                }

                for arg in args {
                    Self::resolve_expression(arg, ctx);
                }
            }
        }
    }
}