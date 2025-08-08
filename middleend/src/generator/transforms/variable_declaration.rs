use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedVariableDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        let mut instructions = vec![];

        if let Some(storage_class) = declaration.storage_class() {
            if *storage_class == AstStorageClass::Static {
                return instructions;
            }
        }

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