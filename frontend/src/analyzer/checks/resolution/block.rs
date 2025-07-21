use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_block(block: &mut TypedBlock, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedBlock, AnalyzerContext<'_, '_>>>::run(block, ctx)
    }
}

impl Factory<(), TypedBlock, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext) {
        ctx.push_scope(true);
        
        for item in block.block_items_mut() {
            Self::resolve_block_item(item, ctx);
        }
        
        ctx.pop_scope();
    }
}