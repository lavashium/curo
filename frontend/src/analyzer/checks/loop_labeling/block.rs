use common::*;
use super::*;

impl LoopLabeling {
    pub fn label_block(block: &mut TypedBlock, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedBlock, AnalyzerContext<'_, '_>>>::run(block, ctx)
    }
}

impl Factory<(), TypedBlock, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext) -> () {
        for item in block.block_items_mut() {
            LoopLabeling::label_block_item(item, ctx);
        }
    }
}