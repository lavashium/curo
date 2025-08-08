use crate::*;
use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedBlock> for GeneratorTransforms<'scp, 'ctx> {
    fn run(block: &mut TypedBlock, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        let mut instructions = vec![];
        for item in block.block_items_mut() {
            instructions.append(&mut Self::run(item, ctx));
        }
        instructions
    }
}