use language::*;
use super::*;

pub fn resolve_statement(
    stmt: &mut AstStatement,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) {
    match stmt {
        AstStatement::Return { expression, .. } => {
            resolve_expression(expression, ctx, map);
        }
        AstStatement::Expression { expression, .. } => {
            resolve_expression(expression, ctx, map);
        }
        AstStatement::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            resolve_expression(condition, ctx, map);
            resolve_statement(then_branch, ctx, map);
            if let Some(else_branch) = else_branch {
                resolve_statement(else_branch, ctx, map);
            }
        }
        AstStatement::Compound { block, .. } => {
            resolve_block(block, ctx, map);
        }
        AstStatement::While { condition, body, .. } => {
            ctx.loop_depth += 1;
            resolve_expression(condition, ctx, map);
            resolve_statement(body, ctx, map);
            ctx.loop_depth -= 1;
        }
        AstStatement::DoWhile { condition, body, .. } => {
            ctx.loop_depth += 1;
            resolve_statement(body, ctx, map);
            resolve_expression(condition, ctx, map);
            ctx.loop_depth -= 1;
        }
        AstStatement::For {
            for_init,
            condition,
            post,
            body,
            ..
        } => {
            ctx.loop_depth += 1;
            let mut loop_map = copy_identifier_map(map);

            match for_init {
                AstForInit::InitDeclaration(var_decl) => {
                    resolve_variable_declaration(var_decl, ctx, &mut loop_map)
                }
                AstForInit::InitExpression(opt) => {
                    if let Some(expr) = opt {
                        resolve_expression(expr, ctx, &loop_map);
                    }
                }
            }
            if let Some(cond) = condition {
                resolve_expression(cond, ctx, &loop_map);
            }
            if let Some(p) = post {
                resolve_expression(p, ctx, &loop_map);
            }
            resolve_statement(body, ctx, &mut loop_map);
            ctx.loop_depth -= 1;
        }
        AstStatement::Break { span, .. } => {
            if ctx.loop_depth == 0 {
                push_error(ctx, *span, DiagnosticKind::Custom("break not in loop".into()));
            }
        }
        AstStatement::Continue { span, .. } => {
            if ctx.loop_depth == 0 {
                push_error(ctx, *span, DiagnosticKind::Custom("continue not in loop".into()));
            }
        }
        _ => ()
    }
}