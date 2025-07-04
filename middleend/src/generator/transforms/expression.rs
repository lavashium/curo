use super::*;
use language::*;

pub trait ExpressionTransform {
    fn transform_expression(&mut self, expression: &AstExpression) -> (Vec<TacInstruction>, TacVal);
}

impl<'a> ExpressionTransform for GeneratorTransforms<'a> {
    fn transform_expression(&mut self, expression: &AstExpression) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            AstExpression::Constant { constant } => {
                let val = TacVal::new_constant(constant.clone());
                (vec![], val)
            }

            AstExpression::Unary { operator, operand } => {
                let (mut instructions, source) = self.transform_expression(operand);

                let destination = self.generator.tempgen.next();

                instructions.push(TacInstruction::new_unary (
                    *operator,
                    source,
                    destination.clone(),
                ));

                (instructions, destination)
            }

            AstExpression::Binary {
                operator,
                left,
                right,
            } => {
                let mut instructions = Vec::new();

                let (mut instructions_left, dest_left) = self.transform_expression(left);
                let (mut instructions_right, dest_right) = self.transform_expression(right);

                let destination = self.generator.tempgen.next();

                instructions.append(&mut instructions_left);
                instructions.append(&mut instructions_right);

                instructions.push(TacInstruction::new_binary (
                    *operator,
                    dest_left,
                    dest_right,
                    destination.clone(),
                ));

                (instructions, destination)
            }
        }
    }
}
