use common::*;
use super::*;

impl Factory<(), TypedBlock, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext) -> () {
        for item in block.block_items_mut() {
            LoopLabeling::run(item, ctx);
        }
    }
}