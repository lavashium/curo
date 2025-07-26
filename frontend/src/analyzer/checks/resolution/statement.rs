use common::*;
use super::*;

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        match stmt {
            TypedStatement::Return { expression, .. } => {
                Self::run(expression, ctx);
            }
            TypedStatement::Expression { expression, .. } => {
                Self::run(expression, ctx);
            }
            TypedStatement::If { condition, then_branch, else_branch, .. } => {
                Self::run(condition, ctx);
                Self::run(&mut **then_branch, ctx);
                if let Some(else_branch) = else_branch {
                    Self::run(&mut **else_branch, ctx);
                }
            }
            TypedStatement::Compound { block, .. } => {
                Self::run(block, ctx);
            }
            TypedStatement::While { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::run(condition, ctx);
                Self::run(&mut **body, ctx);
                ctx.loop_depth -= 1;
            }
            TypedStatement::DoWhile { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::run(&mut **body, ctx);
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
                        if let Some(expr) = expr {
                            Self::run(expr, ctx);
                        }
                    }
                }
                
                if let Some(cond) = condition {
                    Self::run(cond, ctx);
                }
                
                if let Some(p) = post {
                    Self::run(p, ctx);
                }
                
                Self::run(&mut **body, ctx);
                ctx.scope = old_scope;
                ctx.loop_depth -= 1;
            }
            TypedStatement::Break { span, .. } if ctx.loop_depth == 0 => {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    *span,
                    DiagnosticKind::Custom("break not in loop".into()),
                ));
            }
            TypedStatement::Continue { span, .. } if ctx.loop_depth == 0 => {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    *span,
                    DiagnosticKind::Custom("continue not in loop".into()),
                ));
            }
            TypedStatement::Null 
            | TypedStatement::Break { .. } 
            | TypedStatement::Continue { .. } => {}
        }
    }
}