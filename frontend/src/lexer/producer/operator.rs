use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::DiagnosticsManager;
use language::*;

pub struct OperatorProducer;

impl TokenProducer for OperatorProducer {
    fn try_match(_lexer: &mut Lexer, _diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        None
    }
}