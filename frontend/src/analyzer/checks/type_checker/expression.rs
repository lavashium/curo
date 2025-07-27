use common::*;
use super::*;

impl Factory<(), TypedExpression, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(exp: &mut TypedExpression, ctx: &mut AnalyzerContext) {
        let span = exp.get_span();
        match exp {
            TypedExpression::FunctionCall { identifier, args, .. } => {
                let symtable = &ctx.ctx.symtable;
                match symtable.get(identifier) {
                    Some(symbol) => match symbol.ty() {
                        AstType::Int => {
                            ctx.ctx.diagnostics.push(
                                Diagnostic::error(
                                    span,
                                    DiagnosticKind::Semantic(SemanticError::TypeMismatch {
                                        expected: AstType::FunType(args.len()),
                                        found: AstType::Int,
                                    }),
                                ),
                            );
                        }
                        AstType::FunType(param_count) => {
                            if args.len() != *param_count {
                                ctx.ctx.diagnostics.push(
                                    Diagnostic::error(
                                        span,
                                        DiagnosticKind::Semantic(
                                            SemanticError::InvalidFunctionCall {
                                                name: identifier.clone(),
                                                expected_args: *param_count,
                                                found_args: args.len(),
                                            }
                                        ),
                                    ),
                                );
                            } else {
                                for arg in args.iter_mut() {
                                    TypeCheck::run(&mut **arg, ctx);
                                }
                            }
                        }
                    },
                    None => {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(
                                    SemanticError::UndefinedFunction {
                                        name: identifier.clone(),
                                    }
                                ),
                            ),
                        );
                    }
                }
            }

            TypedExpression::Var { identifier, .. } => {
                let symtable = &ctx.ctx.symtable;
                match symtable.get(identifier) {
                    Some(symbol) => match symbol.ty() {
                        AstType::Int => {}
                        AstType::FunType(param_count) => {
                            ctx.ctx.diagnostics.push(
                                Diagnostic::error(
                                    span,
                                    DiagnosticKind::Semantic(SemanticError::TypeMismatch {
                                        expected: AstType::Int,
                                        found: AstType::FunType(*param_count),
                                    }),
                                ),
                            );
                        }
                    },
                    None => {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(
                                    SemanticError::UseOfUndeclared {
                                        name: identifier.clone(),
                                    }
                                ),
                            ),
                        );
                    }
                }
            }

            TypedExpression::Unary { operand, .. } => {
                TypeCheck::run(&mut **operand, ctx);
            }

            TypedExpression::Binary { left, right, .. } => {
                TypeCheck::run(&mut **left, ctx);
                TypeCheck::run(&mut **right, ctx);
            }

            TypedExpression::Assignment { left, right, .. } => {
                TypeCheck::run(&mut **left, ctx);
                TypeCheck::run(&mut **right, ctx);
            }

            TypedExpression::Conditional { condition, then_branch, else_branch, .. } => {
                TypeCheck::run(&mut **condition, ctx);
                TypeCheck::run(&mut **then_branch, ctx);
                TypeCheck::run(&mut **else_branch, ctx);
            }

            TypedExpression::Constant { .. } => {}
        }
    }
}
