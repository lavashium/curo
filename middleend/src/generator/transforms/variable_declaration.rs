use super::*;
use language::*;
use common::*;

impl Factory<Vec<TacInstruction>, TypedVariableDeclaration, TacGenContext<'_, '_>> for GeneratorTransforms{
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut TacGenContext) -> Vec<TacInstruction> {
        let mut instructions = vec![];

        let var_name = declaration.identifier().clone();
        let tac_var = TacVal::new_var(var_name);

        if let Some(init_expr) = declaration.init_mut() {
            let (mut expr_instrs, value) = Self::run(init_expr, ctx);
            instructions.append(&mut expr_instrs);

            instructions.push(TacInstruction::new_copy(value, tac_var));
        }

        instructions
    }
}