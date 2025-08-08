use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedExpression> for TypeCheck<'scp, 'ctx> {
    fn run(exp: &mut TypedExpression, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
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
                                    Self::run_box(arg, ctx);
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
                Self::run_box(operand, ctx);
            }

            TypedExpression::Binary { left, right, .. } => {
                Self::run_box(left, ctx);
                Self::run_box(right, ctx);
            }

            TypedExpression::Assignment { left, right, .. } => {
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
