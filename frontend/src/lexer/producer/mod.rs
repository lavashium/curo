mod comment;
mod constant;
mod keyword;
mod operator;
mod punctuation;
mod whitespace;

use comment::*;
use constant::*;
use keyword::*;
use operator::*;
use punctuation::*;
use whitespace::*;

use crate::lexer::Lexer;
use common::DiagnosticsManager;
use language::*;

macro_rules! auto_nest {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        ($head, auto_nest!($($tail),*))
    };
}

pub trait TokenProducerList {
    fn try_all(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token>;
}

impl TokenProducerList for () {
    fn try_all(_: &mut Lexer, _: &mut DiagnosticsManager) -> Option<Token> {
        None
    }
}

impl<Head: TokenProducer, Tail: TokenProducerList> TokenProducerList for (Head, Tail) {
    fn try_all(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        Head::try_match(lexer, diagnostics).or_else(|| Tail::try_all(lexer, diagnostics))
    }
}

pub type PRODUCERS = auto_nest!(
    WhitespaceProducer,
    CommentProducer,
    KeywordProducer,
    ConstantProducer,
    PunctuationProducer,
    OperatorProducer,
);

pub trait TokenProducer {
    fn try_match(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token>;
}
