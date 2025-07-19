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

use common::*;

pub type PRODUCERS = factory_pipeline!(
    WhitespaceProducer,
    CommentProducer,
    KeywordProducer,
    ConstantProducer,
    PunctuationProducer,
    OperatorProducer,
);