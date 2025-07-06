use super::*;
use language::*;

pub trait FunctionTransform {
    fn transform_function(&mut self, function: &AstFunction) -> TacFunction;
}

impl<'a> FunctionTransform for GeneratorTransforms<'a> {
    fn transform_function(&mut self, function: &AstFunction) -> TacFunction {
        let identifier = function.get_name();
        let statement = todo!();
        let instructions = self.transform_statement(statement);
        TacFunction::new(identifier, instructions)
    }
}
