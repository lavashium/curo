mod lexer;
mod parser;
mod analyzer;
mod tac_generator;
mod x84_64;

pub use lexer::*;
pub use parser::*;
pub use analyzer::*;
pub use tac_generator::*;
pub use x84_64::*;

use common::*;
use crate::*;

pub enum PipelineResult<T> {
    Continue(T),
    Halt(Result<String, ErrCode>),
}

impl<T> Chain<PipelineResult<T>> for PipelineResult<T> {
    fn chain(lhs: PipelineResult<T>, rhs: impl FnOnce() -> PipelineResult<T>) -> PipelineResult<T> {
        match lhs {
            PipelineResult::Continue(_) => rhs(),
            PipelineResult::Halt(res) => PipelineResult::Halt(res),
        }
    }
}