use common::*;
use super::*;

impl Factory<(), TypedExpression, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(exp: &mut TypedExpression, ctx: &mut AnalyzerContext) {
        let span = exp.get_span();
        match exp {
            TypedExpression::FunctionCall { identifier, args, .. } => {
                let symtable = &ctx.ctx.symtable;
                match symtable.get(&identifier) {
                    Some(symbol) => match symbol.ty() {
                        AstType::Int => report_error(
                            ctx, 
                            span, 
                            format!("Tried to use variable '{}' as function", identifier)
                        ),
                        AstType::FunType(param_count) => {
                            if args.len() != *param_count {
                                report_error(
                                    ctx,
                                    span,
                                    format!(
                                        "Function '{}' called with {} arguments (expected {})",
                                        identifier,
                                        args.len(),
                                        param_count
                                    ),
                                );
                            } else {
                                for arg in args.iter_mut() {
                                    TypeCheck::run(&mut **arg, ctx);
                                }
                            }
                        }
                    },
                    None => report_error(
                        ctx, 
                        span, 
                        format!("Use of undeclared function '{}'", identifier)
                    ),
                }
            }
            
            TypedExpression::Var { identifier, .. } => {
                let symtable = &ctx.ctx.symtable;
                match symtable.get(&identifier) {
                    Some(symbol) => match symbol.ty() {
                        AstType::Int => (),
                        AstType::FunType(_) => report_error(
                            ctx,
                            span,
                            format!("Tried to use function '{}' as variable", identifier),
                        ),
                    },
                    None => report_error(
                        ctx, 
                        span, 
                        format!("Use of undeclared variable '{}'", identifier)
                    ),
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
            
            TypedExpression::Conditional { 
                condition, 
                then_branch, 
                else_branch, 
                .. 
            } => {
                TypeCheck::run(&mut **condition, ctx);
                TypeCheck::run(&mut **then_branch, ctx);
                TypeCheck::run(&mut **else_branch, ctx);
            }
            
            TypedExpression::Constant { .. } => (),
        }
    }
}