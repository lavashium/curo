use crate::parser::rules::*;
use language::*;

pub fn parse_continue(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(Continue))?;
    parser.expect(token_punctuation!(Semicolon))?;
    Some(AstStatement::new_continue(
        String::new(),
    ))
}