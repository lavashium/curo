use super::*;
use language::*;

pub trait StatementTransform {
    fn transform_statement(&mut self, statement: &AstStatement) -> Vec<TacInstruction>;
}

impl<'a> StatementTransform for GeneratorTransforms<'a> {
    fn transform_statement(&mut self, statement: &AstStatement) -> Vec<TacInstruction> {
        let mut instructions: Vec<TacInstruction> = Vec::new();
        match statement {
            AstStatement::Return { expression } => {
                let (mut expression, value) = self.transform_expression(expression);
                instructions.append(&mut expression);
                instructions.push(TacInstruction::new_return(value));
            }
            AstStatement::Expression { expression } => {
                let (mut expr_instrs, _) = self.transform_expression(expression);
                instructions.append(&mut expr_instrs);
            }
            AstStatement::If { condition, then_branch, else_branch } => {
                let (mut condition_instrs, condition_res) = self.transform_expression(condition);
                let mut then_instr = self.transform_statement(&then_branch);
                let end_label = self.generator.tempgen.label("end");
                let mut else_label = end_label.clone();
                let mut else_instr = Vec::new();

                if let Some(boxed_else) = else_branch {
                    else_label = self.generator.tempgen.label("else");
                    else_instr = self.transform_statement(boxed_else);
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
            AstStatement::Compound { block } => {
                instructions.append(&mut self.transform_block(block));
            },
            AstStatement::Null => {},
            _ => {}
        }
        return instructions;
    }
}
