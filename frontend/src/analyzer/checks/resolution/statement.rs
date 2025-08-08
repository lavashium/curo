use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedStatement> for IdentifierResolution<'scp, 'ctx> {
    fn run(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match stmt {
            TypedStatement::Return { expression, .. } => {
                Self::run(expression, ctx);
            }
            TypedStatement::Expression { expression, .. } => {
                Self::run(expression, ctx);
            }
            TypedStatement::If { condition, then_branch, else_branch, .. } => {
                Self::run(condition, ctx);
                Self::run_box(then_branch, ctx);
                Self::run_option_box(else_branch, ctx);
            }
            TypedStatement::Compound { block, .. } => {
                Self::run(block, ctx);
            }
            TypedStatement::While { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::run(condition, ctx);
                Self::run_box(body, ctx);
                ctx.loop_depth -= 1;
            }
            TypedStatement::DoWhile { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::run_box(body, ctx);
                Self::run(condition, ctx);
                ctx.loop_depth -= 1;
            }
            TypedStatement::For { for_init, condition, post, body, .. } => {
                ctx.loop_depth += 1;
                let old_scope = ctx.scope.clone();
                ctx.scope = copy_identifier_map(&ctx.scope);
                
                match for_init {
                    TypedForInit::InitDeclaration{ decl, .. } => {
                        Self::run(decl, ctx);
                    }
                    TypedForInit::InitExpression{ expr, .. } => {
                        Self::run_option(expr, ctx);
                    }
                }
                
                Self::run_option(condition, ctx);
                
                Self::run_option(post, ctx);
                
                Self::run_box(body, ctx);
                ctx.scope = old_scope;
                ctx.loop_depth -= 1;
            }
            TypedStatement::Break { span, .. } if ctx.loop_depth == 0 => {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    *span,
                    DiagnosticKind::ControlFlow(ControlFlowError::BreakOutsideLoop),
                ));
            }
            TypedStatement::Continue { span, .. } if ctx.loop_depth == 0 => {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    *span,
                    DiagnosticKind::ControlFlow(ControlFlowError::ContinueOutsideLoop),
                ));
            }
            TypedStatement::Null 
            | TypedStatement::Break { .. } 
            | TypedStatement::Continue { .. } => {}
        }
    }
}