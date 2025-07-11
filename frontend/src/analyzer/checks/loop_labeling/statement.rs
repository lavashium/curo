use language::*;
use super::*;

pub fn label_statement(
    stmt: &mut AstStatement,
    ctx: &mut SemanticContext,
    current_loop: Option<String>,
) {
    match stmt {
        AstStatement::Return { .. } |
        AstStatement::Expression { .. } |
        AstStatement::Null => {}

        AstStatement::If { then_branch, else_branch, .. } => {
            label_statement(then_branch, ctx, current_loop.clone());
            if let Some(else_stmt) = else_branch {
                label_statement(else_stmt, ctx, current_loop.clone());
            }
        }

        AstStatement::Compound { block, .. } => {
            label_block(block, ctx, current_loop.clone());
        }

        AstStatement::Break { label, .. } |
        AstStatement::Continue { label, .. } => {
            *label = current_loop.clone().unwrap_or_else(|| "unlabeled".to_string());
        }

        AstStatement::While { body, label: loop_label, .. } |
        AstStatement::DoWhile { body, label: loop_label, .. } => {
            let new_label = ctx.temp_gen_mut().label("loop");
            *loop_label = new_label.clone();
            label_statement(body, ctx, Some(new_label));
        }

        AstStatement::For { body, label: loop_label, .. } => {
            let new_label = ctx.temp_gen_mut().label("loop");
            *loop_label = new_label.clone();
            label_statement(body, ctx, Some(new_label));
        }
    }
}
