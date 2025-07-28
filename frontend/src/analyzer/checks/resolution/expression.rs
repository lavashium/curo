use common::*;
use super::*;

impl Factory<(), TypedExpression, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(expr: &mut TypedExpression, ctx: &mut AnalyzerContext) {
        match expr {
           TypedExpression::Var { identifier, span, .. } => {
                if let Some(info) = ctx.scope.get(identifier) {
                    *identifier = info.unique_name.clone();
                } else {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::Semantic(SemanticError::UseOfUndeclared {
                            name: identifier.clone(),
                        }),
                    ));
                }
            }
            TypedExpression::Assignment { left, right, span, ..} => {
                if let TypedExpression::Var { .. } = **left {

                } else {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span,
                        DiagnosticKind::Syntax(SyntaxError::InvalidLValue),
                    ));
                }

                Self::run_box(left, ctx);
                Self::run_box(right, ctx);
            }
            TypedExpression::FunctionCall { identifier, args, span, .. } => {
                if let Some(new_fun_name) = ctx.scope.get(identifier) {
                    *identifier = new_fun_name.get_unique_name();
                    for arg in args {
                        Self::run_box(arg, ctx);
                    }
                } else {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        *span, 
                        DiagnosticKind::Semantic(SemanticError::UseOfUndeclared {
                            name: identifier.clone(),
                        }),
                    ));
                }
            }
            TypedExpression::Unary { operand, .. } => {
                Self::run_box(operand, ctx);
            }
            TypedExpression::Binary { left, right, .. } => {
                Self::run_box(left, ctx);
                Self::run_box(right, ctx);
            }
            TypedExpression::Conditional { condition, then_branch, else_branch, .. } => {
                Self::run_box(condition, ctx);
                Self::run_box(then_branch, ctx);
                Self::run_box(else_branch, ctx);
            }
            TypedExpression::Constant { .. } => {}
        }
    }
}