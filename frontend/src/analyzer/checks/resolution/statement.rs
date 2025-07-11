use language::*;
use super::*;

pub fn resolve_statement(
    stmt: &mut AstStatement,
    ctx: &mut SemanticContext,
    map: &mut VariableMap,
) {
    match stmt {
        AstStatement::Return { expression, .. } => {
            resolve_expression(expression, ctx, map);
        }
        AstStatement::Expression { expression, .. } => {
            resolve_expression(expression, ctx, map);
        }
        AstStatement::If { condition, then_branch, else_branch, .. } => {
            resolve_expression(condition, ctx, map);
            resolve_statement(then_branch, ctx, map);
            if let Some(else_branch) = else_branch {
                resolve_statement(else_branch, ctx, map);
            }
        }
        AstStatement::Compound { block, .. } => {
            let mut child_map = copy_variable_map(map);
            resolve_block(block, ctx, &mut child_map);
        }
        AstStatement::Break { .. } | AstStatement::Continue { .. } => {
            if ctx.loop_depth == 0 {
                ctx.diagnostics_mut().push(Diagnostic::error(
                    Span::default(),
                    DiagnosticKind::new_custom("break/continue outside loop".into()),
                ));
            }
        }
        AstStatement::While { condition, body, .. } => {
            resolve_expression(condition, ctx, map);
            ctx.loop_depth += 1;
            resolve_statement(body, ctx, map);
            ctx.loop_depth -= 1;
        }
        AstStatement::DoWhile { condition, body, .. } => {
            ctx.loop_depth += 1;
            resolve_statement(body, ctx, map);
            ctx.loop_depth -= 1;
            resolve_expression(condition, ctx, map);
        }
        AstStatement::For { for_init, condition, post, body, .. } => {
            let mut child_map = copy_variable_map(map);

            match for_init {
                AstForInit::InitDeclaration(decl) => {
                    resolve_declaration(decl, ctx, &mut child_map);
                }
                AstForInit::InitExpression(opt_expr) => {
                    if let Some(expr) = opt_expr {
                        resolve_expression(expr, ctx, &mut child_map);
                    }
                }
            }

            if let Some(cond_expr) = condition {
                resolve_expression(cond_expr, ctx, &child_map);
            }
            if let Some(post_expr) = post {
                resolve_expression(post_expr, ctx, &child_map);
            }

            ctx.loop_depth += 1;
            resolve_statement(body, ctx, &mut child_map);
            ctx.loop_depth -= 1;
        }
        AstStatement::Null => {}
    }
}
