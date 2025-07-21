use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_function_declaration(&mut self, function: &mut TypedFunctionDeclaration) -> TacFunction {
        <Self as Factory<TacFunction, Self, TypedFunctionDeclaration>>::run(self, function)
    }
}

impl<'scp, 'ctx> Factory<TacFunction, Self, TypedFunctionDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, function: &mut TypedFunctionDeclaration) -> TacFunction {
        let identifier = function.get_identifier();
        let args = function.get_params();

        let mut instructions = if let Some(block) = function.body_mut() { 
            driver.transform_block(block) 
        } else {
            vec![]
        };

        match instructions.last() {
            Some(TacInstruction::Return { .. }) => {}
            _ => {
                instructions.push(TacInstruction::Return {
                    val: TacVal::Constant("0".to_string()),
                });
            }
        }

        TacFunction::new(identifier, args, instructions)
    }
}
