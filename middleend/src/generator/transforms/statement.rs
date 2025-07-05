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
        }
        return instructions;
    }
}
