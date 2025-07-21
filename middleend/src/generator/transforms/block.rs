use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_block(&mut self, block: &mut TypedBlock) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, TypedBlock>>::run(self, block)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, TypedBlock> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, block: &mut TypedBlock) -> Vec<TacInstruction> {
        let mut instructions = vec![];
        for item in block.block_items_mut() {
            instructions.append(&mut driver.transform_block_item(item));
        }
        instructions
    }
}