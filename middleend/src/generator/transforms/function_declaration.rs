use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_function_declaration(&mut self, function: &mut AstFunctionDeclaration) -> TacFunction {
        <Self as Factory<TacFunction, Self, AstFunctionDeclaration>>::run(self, function)
    }
}

impl<'scp, 'ctx> Factory<TacFunction, Self, AstFunctionDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, function: &mut AstFunctionDeclaration) -> TacFunction {
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
