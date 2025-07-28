use crate::*;
use super::*;
use language::*;
use common::*;

impl Factory<Vec<TacInstruction>, TypedBlockItem, TacGenContext<'_, '_>> for GeneratorTransforms {
    fn run(item: &mut TypedBlockItem, ctx: &mut TacGenContext) -> Vec<TacInstruction> {
        match item {
            TypedBlockItem::Statement(stmt)   => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(decl) => Self::run(decl, ctx),
        }
    }
}