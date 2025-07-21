use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_statement(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedStatement, AnalyzerContext<'_, '_>>>::run(stmt, ctx)
    }
}

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(stmt: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        match stmt {
            TypedStatement::Return { expression, .. } => {
                Self::resolve_expression(expression, ctx);
            }
            TypedStatement::Expression { expression, .. } => {
                Self::resolve_expression(expression, ctx);
            }
            TypedStatement::If { condition, then_branch, else_branch, .. } => {
                Self::resolve_expression(condition, ctx);
                Self::resolve_statement(then_branch, ctx);
                if let Some(else_branch) = else_branch {
                    Self::resolve_statement(else_branch, ctx);
                }
            }
            TypedStatement::Compound { block, .. } => {
                Self::resolve_block(block, ctx);
            }
            TypedStatement::While { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::resolve_expression(condition, ctx);
                Self::resolve_statement(body, ctx);
                ctx.loop_depth -= 1;
            }
            TypedStatement::DoWhile { condition, body, .. } => {
                ctx.loop_depth += 1;
                Self::resolve_statement(body, ctx);
                Self::resolve_expression(condition, ctx);
                ctx.loop_depth -= 1;
            }
            TypedStatement::For { for_init, condition, post, body, .. } => {
                ctx.loop_depth += 1;
                ctx.push_scope(true);
                
                match for_init {
                    TypedForInit::InitDeclaration{ decl, .. } => {
                        Self::resolve_variable_declaration(decl, ctx);
                    }
                    TypedForInit::InitExpression{ expr, .. } => {
                        if let Some(expr) = expr {
                            Self::resolve_expression(expr, ctx);
                        }
                    }
                }
                
                if let Some(cond) = condition {
                    Self::resolve_expression(cond, ctx);
                }
                
                if let Some(p) = post {
                    Self::resolve_expression(p, ctx);
                }
                
                Self::resolve_statement(body, ctx);
                ctx.pop_scope();
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
            _ => {}
        }
    }
}