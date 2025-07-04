use super::*;
use language::*;

pub trait ExpressionTransform {
    fn transform_expression(&mut self, expression: &AstExpression) -> (Vec<TacInstruction>, TacVal);
}

impl<'a> ExpressionTransform for GeneratorTransforms<'a> {
    fn transform_expression(&mut self, expression: &AstExpression) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            AstExpression::Constant { constant } => {
                let val = TacVal::Constant(constant.clone());
                (vec![], val)
            }

            AstExpression::Unary { operator, operand } => {
                let (mut instructions, source) = self.transform_expression(operand);

                let destination = self.generator.tempgen.next();

                instructions.push(TacInstruction::Unary {
                    operator: *operator,
                    source,
                    destination: destination.clone(),
                });

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

                instructions.push(TacInstruction::Binary {
                    operator: *operator,
                    source1: dest_left,
                    source2: dest_right,
                    destination: destination.clone(),
                });

                (instructions, destination)
            }
        }
    }
}
