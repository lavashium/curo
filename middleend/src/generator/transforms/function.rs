use super::*;
use language::*;

pub trait FunctionTransform {
    fn transform_function(&mut self, function: &AstFunction) -> TacFunction;
}

impl<'a> FunctionTransform for GeneratorTransforms<'a> {
    fn transform_function(&mut self, function: &AstFunction) -> TacFunction {
        let identifier = function.get_name();
        let body_items = function.body();

        let mut instructions = vec![];

        for item in body_items {
            instructions.append(&mut self.transform_block_item(item));
        }

        match instructions.last() {
            Some(TacInstruction::Return { .. }) => {}
            _ => {
                instructions.push(TacInstruction::Return {
                    val: TacVal::Constant("0".to_string()),
                });
            }
        }

        TacFunction::new(identifier, instructions)
    }
}
