use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_statement(&mut self, statement: &mut AstStatement) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, AstStatement>>::run(self, statement)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, AstStatement> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, statement: &mut AstStatement) -> Vec<TacInstruction> {
        let mut instructions: Vec<TacInstruction> = Vec::new();
        match statement {
            AstStatement::Return { expression, .. } => {
                let (mut expression, value) = driver.transform_expression(expression);
                instructions.append(&mut expression);
                instructions.push(TacInstruction::new_return(value));
            }
            AstStatement::Expression { expression, .. } => {
                let (mut expr_instrs, _) = driver.transform_expression(expression);
                instructions.append(&mut expr_instrs);
            }
            AstStatement::If { condition, then_branch, else_branch, .. } => {
                let (mut condition_instrs, condition_res) = driver.transform_expression(condition);
                let mut then_instr = driver.transform_statement(then_branch);
                let end_label = driver.ctx.ctx.tempgen.label("end");
                let mut else_label = end_label.clone();
                let mut else_instr = Vec::new();

                if let Some(boxed_else) = else_branch {
                    else_label = driver.ctx.ctx.tempgen.label("else");
                    else_instr = driver.transform_statement(boxed_else);
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
            AstStatement::Compound { block, .. } => {
                instructions.append(&mut driver.transform_block(block));
            },
            AstStatement::Null => {},
            AstStatement::Break { label, .. } => {
                let jump_label = driver.ctx.ctx.tempgen.loop_label(label, "break");
                instructions.push(TacInstruction::new_jump(jump_label));
            }

            AstStatement::Continue { label, .. } => {
                let jump_label = driver.ctx.ctx.tempgen.loop_label(label, "continue");
                instructions.push(TacInstruction::new_jump(jump_label));
            }

            AstStatement::While { label, condition, body, .. } => {
                let continue_label = driver.ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = driver.ctx.ctx.tempgen.loop_label(label, "break");

                instructions.push(TacInstruction::new_label(continue_label.clone()));

                let (mut cond_instrs, cond_val) = driver.transform_expression(condition);
                instructions.append(&mut cond_instrs);
                instructions.push(TacInstruction::new_jump_if_zero(cond_val.clone(), break_label.clone()));

                let mut body_instrs = driver.transform_statement(body);
                instructions.append(&mut body_instrs);

                instructions.push(TacInstruction::new_jump(continue_label));
                instructions.push(TacInstruction::new_label(break_label));
            }

            AstStatement::DoWhile { label, body, condition, .. } => {
                let start_label = driver.ctx.ctx.tempgen.loop_label(label, "start");
                let continue_label = driver.ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = driver.ctx.ctx.tempgen.loop_label(label, "break");

                instructions.push(TacInstruction::new_label(start_label.clone()));

                instructions.append(&mut driver.transform_statement(body));
                instructions.push(TacInstruction::new_label(continue_label.clone()));

                let (mut cond_instrs, cond_val) = driver.transform_expression(condition);
                instructions.append(&mut cond_instrs);
                instructions.push(TacInstruction::new_jump_if_not_zero(cond_val, start_label));

                instructions.push(TacInstruction::new_label(break_label));
            }

            AstStatement::For { for_init, condition, post, body, label, .. } => {
                let start_label = driver.ctx.ctx.tempgen.loop_label(label, "start");
                let continue_label = driver.ctx.ctx.tempgen.loop_label(label, "continue");
                let break_label = driver.ctx.ctx.tempgen.loop_label(label, "break");

                instructions.append(&mut driver.transform_for_init(for_init));

                instructions.push(TacInstruction::new_label(start_label.clone()));

                if let Some(cond_expr) = condition {
                    let (mut cond_instrs, cond_val) = driver.transform_expression(cond_expr);
                    instructions.append(&mut cond_instrs);
                    instructions.push(TacInstruction::new_jump_if_zero(cond_val, break_label.clone()));
                }

                instructions.append(&mut driver.transform_statement(body));
                instructions.push(TacInstruction::new_label(continue_label.clone()));

                if let Some(post_expr) = post {
                    let (mut post_instrs, _) = driver.transform_expression(post_expr);
                    instructions.append(&mut post_instrs);
                }

                instructions.push(TacInstruction::new_jump(start_label));
                instructions.push(TacInstruction::new_label(break_label));
            }
        }

        instructions
    }
}
