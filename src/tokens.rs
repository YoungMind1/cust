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
                    "Delimiter {} found in line {} and block {}",
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
            Operator::Addition => write!(f, "Addition"),
            Operator::Subtraction => write!(f, "Subtraction"),
            Operator::Multiplication => write!(f, "Multiplication"),
            Operator::Division => write!(f, "Division"),
            Operator::Remainder => write!(f, "Remainder"),
            Operator::Increment => write!(f, "Increment"),
            Operator::Decrement => write!(f, "Decrement"),
            Operator::Equal => write!(f, "Equal"),
            Operator::NotEqual => write!(f, "NotEqual"),
            Operator::Bigger => write!(f, "Bigger"),
            Operator::BiggerOrEqual => write!(f, "BiggerOrEqual"),
            Operator::Smaller => write!(f, "Smaller"),
            Operator::SmallerOrEqual => write!(f, "SmallerOrEqual"),
            Operator::LogicalAnd => write!(f, "LogicalAnd"),
            Operator::LogicalOr => write!(f, "LogicalOr"),
            Operator::BitwiseAnd => write!(f, "BitwiseAnd"),
            Operator::LogicalNegation => write!(f, "LogicalNegation"),
            Operator::BitwiseOr => write!(f, "BitwiseOr"),
            Operator::ExclusiveOr => write!(f, "ExclusiveOr"),
            Operator::BitwiseNegation => write!(f, "BitwiseNegation"),
            Operator::LeftShift => write!(f, "LeftShift"),
            Operator::RightShift => write!(f, "RightShift"),
            Operator::Assignment => write!(f, "Assignment"),
            Operator::AdditionAndAssignment => write!(f, "AdditionAndAssignment"),
            Operator::SubtractionAndAssignment => write!(f, "SubtractionAndAssignment"),
            Operator::MultiplicationAndAssignment => write!(f, "MultiplicationAndAssignment"),
            Operator::DivisionAndAssignment => write!(f, "DivisionAndAssignment"),
            Operator::RemainderAndAssignment => write!(f, "RemainderAndAssignment"),
            Operator::RightShiftAndAssignment => write!(f, "RightShiftAndAssignment"),
            Operator::LeftShiftAndAssignment => write!(f, "LeftShiftAndAssignment"),
            Operator::BitwiseAndAssignment => write!(f, "BitwiseAndAssignment"),
            Operator::BitwiseOrAssignment => write!(f, "BitwiseOrAssignment"),
            Operator::BitwiseExclusiveOrAndAssignment => {
                write!(f, "BitwiseExclusiveOrAndAssignment")
            }
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
            Delimiter::Comma => write!(f, "Comma"),
            Delimiter::SemiColon => write!(f, "SemiColon"),
            Delimiter::OpeningParenthesis => write!(f, "OpeningParenthesis"),
            Delimiter::ClosingParenthesis => write!(f, "ClosingParenthesis"),
            Delimiter::OpeningBracket => write!(f, "OpeningBracket"),
            Delimiter::ClosingBracket => write!(f, "ClosingBracket"),
            Delimiter::OpeningCurlyBracket => write!(f, "OpeningCurlyBracket"),
            Delimiter::ClosingCurlyBracket => write!(f, "ClosingCurlyBracket"),
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
            Keyword::Auto => write!(f, "Auto"),
            Keyword::Break => write!(f, "Break"),
            Keyword::Case => write!(f, "Case"),
            Keyword::Char => write!(f, "Char"),
            Keyword::Const => write!(f, "Const"),
            Keyword::Continue => write!(f, "Continue"),
            Keyword::Default => write!(f, "Default"),
            Keyword::Do => write!(f, "Do"),
            Keyword::Double => write!(f, "Double"),
            Keyword::Else => write!(f, "Else"),
            Keyword::Enum => write!(f, "Enum"),
            Keyword::Extern => write!(f, "Extern"),
            Keyword::Float => write!(f, "Float"),
            Keyword::For => write!(f, "For"),
            Keyword::Goto => write!(f, "Goto"),
            Keyword::If => write!(f, "If"),
            Keyword::Inline => write!(f, "Inline"),
            Keyword::Int => write!(f, "Int"),
            Keyword::Long => write!(f, "Long"),
            Keyword::Register => write!(f, "Register"),
            Keyword::Restrict => write!(f, "Restrict"),
            Keyword::Return => write!(f, "Return"),
            Keyword::Short => write!(f, "Short"),
            Keyword::Signed => write!(f, "Signed"),
            Keyword::Sizeof => write!(f, "Sizeof"),
            Keyword::Static => write!(f, "Static"),
            Keyword::Struct => write!(f, "Struct"),
            Keyword::Switch => write!(f, "Switch"),
            Keyword::Typedef => write!(f, "Typedef"),
            Keyword::Union => write!(f, "Union"),
            Keyword::Unsigned => write!(f, "Unsigned"),
            Keyword::Void => write!(f, "Void"),
            Keyword::Volatile => write!(f, "Volatile"),
            Keyword::Wihle => write!(f, "Wihle"),
            Keyword::_Bool => write!(f, "_Bool"),
            Keyword::_Complex => write!(f, "_Complex"),
            Keyword::_Imaginary => write!(f, "_Imaginary"),
        }
    }
}
