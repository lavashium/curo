use common::*;
use super::*;

impl LoopLabeling {
    pub fn label_block_item(block_item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>>>::run(block_item, ctx)
    }
}

impl Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(block_item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) -> () {
        match block_item {
            TypedBlockItem::Statement(stmt) => Self::label_statement(stmt, ctx),
            TypedBlockItem::Declaration(_) => {},
        }
    }
}