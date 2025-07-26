use common::*;
use super::*;

impl Factory<(), TypedBlock, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext) {
        for item in block.block_items_mut() {
            TypeCheck::run(item, ctx);
        }
    }
}