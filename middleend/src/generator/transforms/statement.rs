use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedStatement> for GeneratorTransforms<'scp, 'ctx> {
    fn run(statement: &mut TypedStatement, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        let mut instructions: Vec<TacInstruction> = Vec::new();
        match statement {
            TypedStatement::Return { expression, .. } => {
                let (mut expression, value) = Self::run(expression, ctx);
                instructions.append(&mut expression);
                instructions.push(TacInstruction::new_return(value));
            }
            TypedStatement::Expression { expression, .. } => {
                let (mut expr_instrs, _) = Self::run(expression, ctx);
                instructions.append(&mut expr_instrs);
            }
            TypedStatement::If { condition, then_branch, else_branch, .. } => {
                let (mut condition_instrs, condition_res) = Self::run(condition, ctx);
                let mut then_instr = Self::run_box(then_branch, ctx);
                let end_label = ctx.ctx.tempgen.label("end");
                let mut else_label = end_label.clone();
                let mut else_instr = Vec::new();

                if let Some(boxed_else) = else_branch {
                    else_label = ctx.ctx.tempgen.label("else");
                    else_instr = Self::run_box(boxed_else, ctx);
                }

                instructions.append(&mut condition_instrs);
                instructions.push(TacInstruction::new_jump_if_zero(
                    condition_res,
                    else_label.clone(),
                ));
                instructions.append(&mut then_instr);

                if else_label != end_label {
                    instructions.push(TacInstruction::new_jump(end_label.clone()));
                }

                instructions.push(TacInstruction::new_label(else_label));
                
                instructions.append(&mut else_instr);

                instructions.push(TacInstruction::new_label(end_label));
            }
            TypedStatement::Compound { block, .. } => {
                instructions.append(&mut Self::run(block, ctx));
            },
            TypedStatement::Null => {},
            TypedStatement::Break { label, .. } => {
                let jump_label = ctx.ctx.tempgen.loop_label(label, "break");
                instructions.push(TacInstruction::new_jump(jump_label));
            }

            TypedStatement::Continue { label, .. } => {
                let jump_label = ctx.ctx.tempgen.loop_label(label, "continue");
                instructions.push(TacInstruction::new_jump(jump_label));
            }

            TypedStatement::While { label, condition, body, .. } => {
                let continue_label = ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = ctx.ctx.tempgen.loop_label(label, "break");

                instructions.push(TacInstruction::new_label(continue_label.clone()));

                let (mut cond_instrs, cond_val) = Self::run(condition, ctx);
                instructions.append(&mut cond_instrs);
                instructions.push(TacInstruction::new_jump_if_zero(cond_val.clone(), break_label.clone()));

                let mut body_instrs = Self::run_box(body, ctx);
                instructions.append(&mut body_instrs);

                instructions.push(TacInstruction::new_jump(continue_label));
                instructions.push(TacInstruction::new_label(break_label));
            }

            TypedStatement::DoWhile { label, body, condition, .. } => {
                let start_label = ctx.ctx.tempgen.loop_label(label, "start");
                let continue_label = ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = ctx.ctx.tempgen.loop_label(label, "break");

                instructions.push(TacInstruction::new_label(start_label.clone()));

                instructions.append(&mut Self::run_box(body, ctx));
                instructions.push(TacInstruction::new_label(continue_label.clone()));

                let (mut cond_instrs, cond_val) = Self::run(condition, ctx);
                instructions.append(&mut cond_instrs);
                instructions.push(TacInstruction::new_jump_if_not_zero(cond_val, start_label));

                instructions.push(TacInstruction::new_label(break_label));
            }

            TypedStatement::For { for_init, condition, post, body, label, .. } => {
                let start_label = ctx.ctx.tempgen.loop_label(label, "start");
                let continue_label = ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = ctx.ctx.tempgen.loop_label(label, "break");

                instructions.append(&mut Self::run(for_init, ctx));

                instructions.push(TacInstruction::new_label(start_label.clone()));

                if let Some(cond_expr) = condition {
                    let (mut cond_instrs, cond_val) = Self::run(cond_expr, ctx);
                    instructions.append(&mut cond_instrs);
                    instructions.push(TacInstruction::new_jump_if_zero(cond_val, break_label.clone()));
                }

                instructions.append(&mut Self::run_box(body, ctx));
                instructions.push(TacInstruction::new_label(continue_label.clone()));

                if let Some(post_expr) = post {
                    let (mut post_instrs, _) = Self::run(post_expr, ctx);
                    instructions.append(&mut post_instrs);
                }

                instructions.push(TacInstruction::new_jump(start_label));
                instructions.push(TacInstruction::new_label(break_label));
            }
        }

        instructions
    }
}
