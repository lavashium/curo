use language::*;
use super::*;

pub fn label_statement(
    stmt: &mut AstStatement,
    ctx: &mut SemanticContext,
    current_loop: &Option<String>,
) {
    match stmt {
        AstStatement::Return { .. }
        | AstStatement::Expression { .. }
        | AstStatement::Null => {}

        AstStatement::If { then_branch, else_branch, .. } => {
            label_statement(then_branch, ctx, current_loop);
            if let Some(else_stmt) = else_branch {
                label_statement(else_stmt, ctx, current_loop);
            }
        }

        AstStatement::Compound { block, .. } => {
            super::block::label_block(block, ctx, current_loop);
        }

        AstStatement::Break { label, .. }
        | AstStatement::Continue { label, .. } => {
            *label = current_loop.clone().unwrap_or_else(|| "unlabeled".to_string());
        }

        AstStatement::While { body, label, .. }
        | AstStatement::DoWhile { body, label, .. }
        | AstStatement::For { body, label, .. } => {
            let new_lbl = ctx.temp_gen_mut().label("loop");
            *label = new_lbl.clone();
            label_statement(body, ctx, &Some(new_lbl));
        }
    }
}