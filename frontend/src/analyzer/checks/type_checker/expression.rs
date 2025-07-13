use language::*;
use super::*;

pub fn typecheck_expression(
    expr: &mut AstExpression,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    match expr {
        AstExpression::Var { identifier, span } => {
            if let Some(sym) = symbols.get(identifier) {
                if sym.ty != Type::Int {
                    push_error(
                        ctx,
                        *span,
                        DiagnosticKind::Custom(format!("Function used as variable: {}", identifier)),
                    );
                }
            } else {
                push_error(
                    ctx,
                    *span,
                    DiagnosticKind::UseOfUndeclared { name: identifier.clone() },
                );
            }
        }
        AstExpression::FunctionCall { identifier, args, span } => {
            if let Some(sym) = symbols.get(identifier) {
                match sym.ty {
                    Type::FunType(n) => {
                        if n != args.len() {
                            push_error(
                                ctx,
                                *span,
                                DiagnosticKind::Custom(format!("Wrong argument count for {}: got {}, expected {}", identifier, args.len(), n)),
                            );
                        }
                    }
                    Type::Int => {
                        push_error(
                            ctx,
                            *span,
                            DiagnosticKind::Custom(format!("Variable used as function: {}", identifier)),
                        );
                    }
                }
            } else {
                push_error(
                    ctx,
                    *span,
                    DiagnosticKind::UseOfUndeclared { name: identifier.clone() },
                );
            }
            for arg in args {
                typecheck_expression(arg, symbols, ctx);
            }
        }
        AstExpression::Constant { .. } => {}
        AstExpression::Unary { operand, .. } => typecheck_expression(operand, symbols, ctx),
        AstExpression::Binary { left, right, .. } => {
            typecheck_expression(left, symbols, ctx);
            typecheck_expression(right, symbols, ctx);
        }
        AstExpression::Assignment { left, right, span } => {
            match left.as_ref() {
                AstExpression::Var { .. } => {}
                _ => {
                    push_error(
                        ctx,
                        *span,
                        DiagnosticKind::Custom("Invalid lvalue".to_string()),
                    );
                }
            }
            typecheck_expression(left, symbols, ctx);
            typecheck_expression(right, symbols, ctx);
        }
        AstExpression::Conditional { condition, then_branch, else_branch, .. } => {
            typecheck_expression(condition, symbols, ctx);
            typecheck_expression(then_branch, symbols, ctx);
            typecheck_expression(else_branch, symbols, ctx);
        }
    }
}