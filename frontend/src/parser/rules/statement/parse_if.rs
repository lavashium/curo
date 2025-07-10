use crate::parser::rules::*;
use language::*;

pub fn parse_if(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(If))?;
    parser.expect(token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression()?;
    parser.expect(token_punctuation!(CloseParen))?;
    let then_branch = parser.parse_statement()?;
    let else_branch = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Keyword(KeywordKind::Else) => {
            parser.expect(token_keyword!(Else))?;
            Some(Box::new(parser.parse_statement()?))
        },
        _ => None,
    };

    Some(AstStatement::new_if(
        condition,
        Box::new(then_branch),
        else_branch,
    ))
}
