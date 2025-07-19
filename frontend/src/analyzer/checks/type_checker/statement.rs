use language::*;
use super::*;

pub fn typecheck_statement(
    stmt: &mut AstStatement,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    match stmt {
        AstStatement::Return { expression, span, .. } => {
            typecheck_expression(expression, symbols, ctx);
        }
        AstStatement::Expression { expression, .. } => {
            typecheck_expression(expression, symbols, ctx);
        }
        AstStatement::Compound { block, .. } => {
            typecheck_block(block, symbols, ctx);
        }
        AstStatement::If { condition, then_branch, else_branch, .. } => {
            typecheck_expression(condition, symbols, ctx);
            typecheck_statement(then_branch, symbols, ctx);
            if let Some(else_b) = else_branch {
                typecheck_statement(else_b, symbols, ctx);
            }
        }
        AstStatement::While { condition, body, .. }
        | AstStatement::DoWhile { condition, body, .. } => {
            typecheck_expression(condition, symbols, ctx);
            typecheck_statement(body, symbols, ctx);
        }
        AstStatement::For { for_init, condition, post, body, .. } => {
            symbols.push_scope();

            match for_init {
                AstForInit::InitDeclaration{decl, span} => {
                    symbols.add_variable(decl.identifier(), *span, ctx);
                }
                AstForInit::InitExpression{expr, ..} => {
                    if let Some(expr) = expr {
                        typecheck_expression(expr, symbols, ctx);
                    }
                }
            }

            if let Some(cond) = condition {
                typecheck_expression(cond, symbols, ctx);
            }

            if let Some(post_expr) = post {
                typecheck_expression(post_expr, symbols, ctx);
            }

            typecheck_statement(body, symbols, ctx);

            symbols.pop_scope();
        }
        _ => {}
    }
}