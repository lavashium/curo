use common::*;
use super::*;

impl Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(block_item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) -> () {
        match block_item {
            TypedBlockItem::Statement(stmt) => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(_) => {},
        }
    }
}