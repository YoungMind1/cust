use std::fmt::Display;

pub struct Token {
    pub line: usize,
    pub block: usize,
    pub token_type: TokenType,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::Identifier(identifier) => {
                write!(
                    f,
                    "Identifier \"{}\" in line {} and block {}",
                    identifier, self.line, self.block
                )
            }
            TokenType::Comment(comment) => {
                write!(
                    f,
                    "Comment \"{}\" starting from line {} and block {}",
                    comment, self.line, self.block
                )
            }
            TokenType::Operator(operator) => {
                write!(
                    f,
                    "Operator \"{}\" found in line {} and block {}",
                    operator, self.line, self.block
                )
            }
            TokenType::Literal(literal) => {
                write!(
                    f,
                    "Literal \"{}\" found in line {} and block {}",
                    literal, self.line, self.block
                )
            }
            TokenType::Keyword(keyword) => {
                write!(
                    f,
                    "keyword \"{}\" found in line {} and block {}",
                    keyword, self.line, self.block
                )
            }
            TokenType::Number(number) => {
                write!(
                    f,
                    "Number \"{}\" found in line {} and block {}",
                    number, self.line, self.block
                )
            }
            TokenType::Delimiter(delimiter) => {
                write!(
                    f,
                    "Delimiter \"{}\" found in line {} and block {}",
                    delimiter, self.line, self.block
                )
            }
        }
    }
}

pub enum TokenType {
    Identifier(String),
    Comment(String),
    Operator(Operator),
    Literal(String),
    Keyword(Keyword),
    Number(String),
    Delimiter(Delimiter),
}

pub enum Operator {
    // Arithmetic Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    Increment,
    Decrement,

    // Relational Operators
    Equal,
    NotEqual,
    Bigger,
    BiggerOrEqual,
    Smaller,
    SmallerOrEqual,

    // Logical and Bitwise Operators
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    LogicalNegation,
    BitwiseOr,
    ExclusiveOr,
    BitwiseNegation,
    LeftShift,
    RightShift,

    // Assignment Operators
    Assignment,
    AdditionAndAssignment,
    SubtractionAndAssignment,
    MultiplicationAndAssignment,
    DivisionAndAssignment,
    RemainderAndAssignment,
    RightShiftAndAssignment,
    LeftShiftAndAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseExclusiveOrAndAssignment,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Remainder => write!(f, "%"),
            Operator::Increment => write!(f, "++"),
            Operator::Decrement => write!(f, "--"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::Bigger => write!(f, ">"),
            Operator::BiggerOrEqual => write!(f, ">="),
            Operator::Smaller => write!(f, "<"),
            Operator::SmallerOrEqual => write!(f, "<="),
            Operator::LogicalAnd => write!(f, "&&"),
            Operator::LogicalOr => write!(f, "||"),
            Operator::BitwiseAnd => write!(f, "&"),
            Operator::LogicalNegation => write!(f, "!"),
            Operator::BitwiseOr => write!(f, "|"),
            Operator::ExclusiveOr => write!(f, "^"),
            Operator::BitwiseNegation => write!(f, "~"),
            Operator::LeftShift => write!(f, "<<"),
            Operator::RightShift => write!(f, ">>"),
            Operator::Assignment => write!(f, "="),
            Operator::AdditionAndAssignment => write!(f, "+="),
            Operator::SubtractionAndAssignment => write!(f, "-="),
            Operator::MultiplicationAndAssignment => write!(f, "*="),
            Operator::DivisionAndAssignment => write!(f, "/="),
            Operator::RemainderAndAssignment => write!(f, "%="),
            Operator::RightShiftAndAssignment => write!(f, ">>="),
            Operator::LeftShiftAndAssignment => write!(f, "<<="),
            Operator::BitwiseAndAssignment => write!(f, "&="),
            Operator::BitwiseOrAssignment => write!(f, "|="),
            Operator::BitwiseExclusiveOrAndAssignment => write!(f, "^="),
        }
    }
}
pub enum Delimiter {
    Comma,
    SemiColon,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimiter::Comma => write!(f, ","),
            Delimiter::SemiColon => write!(f, ";"),
            Delimiter::OpeningParenthesis => write!(f, "("),
            Delimiter::ClosingParenthesis => write!(f, ")"),
            Delimiter::OpeningBracket => write!(f, "["),
            Delimiter::ClosingBracket => write!(f, "]"),
            Delimiter::OpeningCurlyBracket => write!(f, "{{"),
            Delimiter::ClosingCurlyBracket => write!(f, "}}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    Wihle,
    _Bool,
    _Complex,
    _Imaginary,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Auto => write!(f, "auto"),
            Keyword::Break => write!(f, "break"),
            Keyword::Case => write!(f, "case"),
            Keyword::Char => write!(f, "char"),
            Keyword::Const => write!(f, "const"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Default => write!(f, "default"),
            Keyword::Do => write!(f, "do"),
            Keyword::Double => write!(f, "double"),
            Keyword::Else => write!(f, "else"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::Extern => write!(f, "extern"),
            Keyword::Float => write!(f, "float"),
            Keyword::For => write!(f, "for"),
            Keyword::Goto => write!(f, "goto"),
            Keyword::If => write!(f, "if"),
            Keyword::Inline => write!(f, "inline"),
            Keyword::Int => write!(f, "int"),
            Keyword::Long => write!(f, "long"),
            Keyword::Register => write!(f, "register"),
            Keyword::Restrict => write!(f, "restrict"),
            Keyword::Return => write!(f, "return"),
            Keyword::Short => write!(f, "short"),
            Keyword::Signed => write!(f, "signed"),
            Keyword::Sizeof => write!(f, "sizeof"),
            Keyword::Static => write!(f, "static"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Switch => write!(f, "switch"),
            Keyword::Typedef => write!(f, "typedef"),
            Keyword::Union => write!(f, "union"),
            Keyword::Unsigned => write!(f, "unsigned"),
            Keyword::Void => write!(f, "void"),
            Keyword::Volatile => write!(f, "volatile"),
            Keyword::Wihle => write!(f, "wihle"),
            Keyword::_Bool => write!(f, "_Bool"),
            Keyword::_Complex => write!(f, "_Complex"),
            Keyword::_Imaginary => write!(f, "_Imaginary"),
        }
    }
}
