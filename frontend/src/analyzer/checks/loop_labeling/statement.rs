use common::*;
use super::*;

impl Factory<(), TypedStatement, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(statement: &mut TypedStatement, ctx: &mut AnalyzerContext) -> () {
        match statement {
            TypedStatement::Return { .. }
            | TypedStatement::Expression { .. }
            | TypedStatement::Null => {}

            TypedStatement::If { then_branch, else_branch, .. } => {
                Self::run_box(then_branch, ctx);
                Self::run_option_box(else_branch, ctx);
            }

            TypedStatement::Compound { block, .. } => {
                Self::run(block, ctx);
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
                Self::run_box(body, ctx);
                ctx.current_loop = old_loop;
            }
        }
    }
}