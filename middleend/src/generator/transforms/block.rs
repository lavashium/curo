use crate::*;
use super::*;
use language::*;
use common::*;

impl Factory<Vec<TacInstruction>, TypedBlock, TacGenContext<'_, '_>> for GeneratorTransforms {
    fn run(block: &mut TypedBlock, ctx: &mut TacGenContext) -> Vec<TacInstruction> {
        let mut instructions = vec![];
        for item in block.block_items_mut() {
            instructions.append(&mut Self::run(item, ctx));
        }
        instructions
    }
}