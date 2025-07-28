use super::*;
use language::*;
use common::*;

impl Factory<TacFunction, TypedFunctionDeclaration, TacGenContext<'_, '_>> for GeneratorTransforms {
    fn run(function: &mut TypedFunctionDeclaration, ctx: &mut TacGenContext) -> TacFunction {
        let identifier = function.get_identifier();
        let args = function.get_params();

        let mut instructions = if let Some(block) = function.body_mut() { 
            Self::run(block, ctx) 
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
