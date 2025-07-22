use common::*;
use super::*;

impl LoopLabeling {
    pub fn label_statement(statement: &mut TypedStatement, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedStatement, AnalyzerContext<'_, '_>>>::run(statement, ctx)
    }
}

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(statement: &mut TypedStatement, ctx: &mut AnalyzerContext) -> () {
        match statement {
            TypedStatement::Return { .. }
            | TypedStatement::Expression { .. }
            | TypedStatement::Null => {}

            TypedStatement::If { then_branch, else_branch, .. } => {
                Self::label_statement(then_branch, ctx);
                if let Some(else_stmt) = else_branch {
                    Self::label_statement(else_stmt, ctx);
                }
            }

            TypedStatement::Compound { block, .. } => {
                Self::label_block(block, ctx);
            }

            TypedStatement::Break { label, .. }
            | TypedStatement::Continue { label, .. } => {
                *label = ctx.get_current_loop().unwrap_or_else(|| "unlabeled".to_string());
            }

            TypedStatement::While { body, label, .. }
            | TypedStatement::DoWhile { body, label, .. }
            | TypedStatement::For { body, label, .. } => {
                let new_lbl = ctx.ctx.tempgen.label("loop");
                *label = new_lbl.clone();
                let old_loop = ctx.get_current_loop();
                ctx.current_loop = Some(new_lbl);
                Self::label_statement(body, ctx);
                ctx.current_loop = old_loop;
            }
        }
    }
}