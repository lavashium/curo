use crate::token::*;
use crate::ast::*;
use super::rules;

pub struct Parser {
    pub source_tokens: TokenStream
}

impl Parser {
    pub fn new(source_tokens: TokenStream) -> Self {
        Parser { 
            source_tokens
        }
    }

    pub fn parse(&mut self) -> Program {
        rules::parse_program(self).unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_simple_program() {
        let source_code = "int main(void) {return 2;}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        let program = parser.parse();

        assert_eq!(
            program,
            ast_program!(
                ast_function!(
                    "main",
                    ast_statement_return!(
                        ast_expression_constant!(
                            "2"
                        )
                    )
                )
            )
        )

    }

    #[test]
    fn test_empty_input_should_fail() {
        let source_code = "";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_missing_return_semicolon_should_fail() {
        let source_code = "int main(void) {return 2}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_invalid_function_name_should_fail() {
        let source_code = "int 123(void) {return 2;}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_extra_tokens_after_program_should_fail() {
        let source_code = "int main(void) {return 2;} int x;";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_void_function_with_no_return_should_fail() {
        let source_code = "int main(void) {}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_return_non_constant_should_fail() {
        let source_code = "int main(void) {return x;}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        assert!(rules::parse_program(&mut parser).is_none());
    }

    #[test]
    fn test_whitespace_and_newlines() {
        let source_code = "int   main  (  void )  {\n\treturn   42 ;\n}";
        let mut lexer = Lexer::new(source_code);
        let token_stream = lexer.parse();
        let mut parser = Parser::new(token_stream);
        let program = parser.parse();
        assert_eq!(
            program,
            ast_program!(
                ast_function!(
                    "main",
                    ast_statement_return!(
                        ast_expression_constant!(
                            "42"
                        )
                    )
                )
            )
        );
    }
}