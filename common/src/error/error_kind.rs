use language::*;
use constructors::constructors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
    ControlFlow(ControlFlowError),
    Preprocessor(PreprocessorError),
    Warning(Warning),
    Internal(InternalError),
    Custom(CustomError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexicalError {
    UnknownToken {
        token: Token
    },
    UnexpectedEof,
    UnterminatedMultilineComment,
    UnterminatedStringLiteral,
    UnterminatedCharLiteral,
    InvalidNumberLiteral{
        literal: String
    },
    InvalidCharLiteral{
        literal: String
    },
    NonAsciiCharacter
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedToken {
        found: Token,
        expected: Vec<TokenKind>,
    },
    ExpectedToken {
        expected: TokenKind,
        found: Token,
    },
    UnexpectedDeclaration,
    UnexpectedStatement,
    InvalidLValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticError {
    DuplicateDeclaration {
        name: String 
    },
    UseOfUndeclared {
        name: String
    },
    ConflictingDeclarations {
        name: String
    },
    TypeMismatch { 
        expected: AstType, 
        found: AstType 
    },
    InvalidType {
        ty: AstType
    },
    IncompleteType {
        ty: AstType
    },
    InvalidAssignment,
    InvalidCast {
        from: AstType,
        to: AstType
    },
    InvalidFunctionCall {
        name: String,
        expected_args: usize,
        found_args: usize,
    },
    NonConstInitializer,
    ReturnTypeMismatch {
        expected: AstType,
        found: AstType
    },
    VoidValueUsed,
    InvalidArraySubscript,
    IncompatiblePointerTypes,
    RedeclaredSymbol { 
        name: String 
    },
    UndefinedFunction {
        name: String
    },
    FunctionRedefinition {
        name: String
    },
    NestedFunctionDefinition,
    InvalidStorageSpecifier,
    ConflictingStorageSpecifiers,
    InvalidTypeSpecifierCount {
        expected: usize,
        found: usize,
    },
    MissingTypeSpecifier,
    InvalidStorageSpecifierLocation,
    ConflictingTypeSpecifiers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowError {
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ReturnOutsideFunction,
    CaseOutsideSwitch,
    DuplicateCaseLabel,
    DefaultLabelAlreadyUsed,
    GotoUndefinedLabel {
        label: String
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocessorError {
    Message(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warning {
    UnusedVariable {
        name: String
    },
    UnusedFunction {
        name: String
    },
    VariableShadowing {
        name: String 
    },
    ImplicitConversion {
        from: String,
        to: String 
    },
    SignedUnsignedComparison,
    Message(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalError {
    Message(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomError {
    Message(String)
}