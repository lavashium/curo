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

pub type PRODUCERS<'scp, 'ctx> = factory_list!(
    WhitespaceProducer<'scp, 'ctx>,
    CommentProducer<'scp, 'ctx>,
    KeywordProducer<'scp, 'ctx>,
    ConstantProducer<'scp, 'ctx>,
    PunctuationProducer<'scp, 'ctx>,
    OperatorProducer<'scp, 'ctx>,
);