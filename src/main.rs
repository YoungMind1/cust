use std::{collections::HashMap, env::args, fmt::Display, fs, path::Path, process::ExitCode};

use regex::Regex;

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        println!("{}", help_text());
        return ExitCode::FAILURE;
    }

    if args.get(1).unwrap() == "--help" {
        println!("{}", help_text());
        return ExitCode::FAILURE;
    }

    let file_path = Path::new(args.get(1).unwrap());
    let file = fs::read_to_string(file_path).unwrap();

    let keywords: HashMap<&str, Keyword> = HashMap::from([
        ("auto", Keyword::Auto),
        ("break", Keyword::Break),
        ("case", Keyword::Case),
        ("char", Keyword::Char),
        ("const", Keyword::Const),
        ("continue", Keyword::Continue),
        ("default", Keyword::Default),
        ("do", Keyword::Do),
        ("double", Keyword::Double),
        ("else", Keyword::Else),
        ("enum", Keyword::Enum),
        ("extern", Keyword::Extern),
        ("float", Keyword::Float),
        ("for", Keyword::For),
        ("goto", Keyword::Goto),
        ("if", Keyword::If),
        ("inline", Keyword::Inline),
        ("int", Keyword::Int),
        ("long", Keyword::Long),
        ("register", Keyword::Register),
        ("restrict", Keyword::Restrict),
        ("return", Keyword::Return),
        ("short", Keyword::Short),
        ("signed", Keyword::Signed),
        ("sizeof", Keyword::Sizeof),
        ("static", Keyword::Static),
        ("struct", Keyword::Struct),
        ("switch", Keyword::Switch),
        ("typedef", Keyword::Typedef),
        ("union", Keyword::Union),
        ("unsigned", Keyword::Unsigned),
        ("void", Keyword::Void),
        ("volatile", Keyword::Volatile),
        ("while", Keyword::Wihle),
        ("_Bool", Keyword::_Bool),
        ("_Complex", Keyword::_Complex),
        ("_Imaginary", Keyword::_Imaginary),
    ]);

    let operators: HashMap<&str, Operator> = HashMap::from([
        // Arithmetic Operators
        ("+", Operator::Addition),
        ("-", Operator::Subtraction),
        ("*", Operator::Multiplication),
        ("/", Operator::Division),
        ("%", Operator::Remainder),
        ("++", Operator::Increment),
        ("--", Operator::Decrement),
        // Relational Operators
        ("==", Operator::Equal),
        ("!=", Operator::NotEqual),
        (">", Operator::Bigger),
        (">=", Operator::BiggerOrEqual),
        ("<", Operator::Smaller),
        ("<=", Operator::SmallerOrEqual),
        // Logical and Bitwise Operators
        ("&&", Operator::LogicalAnd),
        ("||", Operator::LogicalOr),
        ("&", Operator::BitwiseAnd),
        ("!", Operator::LogicalNegation),
        ("|", Operator::BitwiseOr),
        ("^", Operator::ExclusiveOr),
        ("~", Operator::BitwiseNegation),
        ("<<", Operator::LeftShift),
        (">>", Operator::RightShift),
        // Assignment Operators
        ("=", Operator::Assignment),
        ("+=", Operator::AdditionAndAssignment),
        ("-=", Operator::SubtractionAndAssignment),
        ("*=", Operator::MultiplicationAndAssignment),
        ("/=", Operator::DivisionAndAssignment),
        ("%=", Operator::RemainderAndAssignment),
        (">>=", Operator::RightShiftAndAssignment),
        ("<<=", Operator::LeftShiftAndAssignment),
        ("&=", Operator::BitwiseAndAssignment),
        ("|=", Operator::BitwiseOrAssignment),
        ("^=", Operator::BitwiseExclusiveOrAndAssignment),
    ]);

    let delimiters: HashMap<&str, Delimiter> = HashMap::from([
        (",", Delimiter::Comma),
        (";", Delimiter::SemiColon),
        ("(", Delimiter::OpeningParenthesis),
        (")", Delimiter::ClosingParenthesis),
        ("[", Delimiter::OpeningBracket),
        ("]", Delimiter::ClosingBracket),
        ("{", Delimiter::OpeningCurlyBracket),
        ("}", Delimiter::ClosingCurlyBracket),
    ]);

    let mut starting_index = 0;
    let mut ending_index = 0;

    let mut tokens: Vec<Token> = vec![];

    let mut is_inside_block_comments = false;
    let mut is_inside_linear_comments = false;
    let mut is_inside_literal = false;
    let mut line = 1;
    let mut block = 1;
    let characters: Vec<char> = file.chars().collect();
    let mut index = 0;
    let identifer_regex = Regex::new(r"^([a-zA-Z_])(0-9a-zA-Z_)*").unwrap();
    while index + 1 != characters.len() {
        let character = characters.get(index).unwrap();
        if is_inside_linear_comments {
            if character == &'\n' {
                is_inside_linear_comments = false;
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Comment(
                        characters
                            .get(starting_index..(starting_index + ending_index))
                            .unwrap()
                            .iter()
                            .collect(),
                    ),
                });
                line += 1;
                block = 1;
                starting_index = 0;
                ending_index = 0;
            } else {
                ending_index += 1;
                block += 1;
            }
            index += 1;
            continue;
        } else if is_inside_block_comments {
            if character == &'*' && characters.get(index + 1).unwrap() == &'/' {
                is_inside_block_comments = false;
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Comment(
                        characters
                            .get(starting_index..(starting_index + ending_index))
                            .unwrap()
                            .iter()
                            .collect(),
                    ),
                });
                starting_index = 0;
                block += 2;
                ending_index = 0;
            } else {
                if *character == '\n' {
                    line += 1;
                }
                block += 1;
                ending_index += 1;
            }
            index += 1;
            continue;
        } else if is_inside_literal {
            if character == &'"' {
                is_inside_literal = false;
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Literal(
                        characters
                            .get(starting_index..(starting_index + ending_index))
                            .unwrap()
                            .iter()
                            .collect(),
                    ),
                });
                starting_index = 0;
                block += 1;
                ending_index = 0;
            } else {
                if *character == '\n' {
                    line += 1;
                }
                block += 1;
                ending_index += 1;
            }
            index += 1;
            continue;
        }
        if character.is_whitespace() {
            index += 1;
            block += 1;
            continue;
        } else if character == &'/' && characters.get(index + 1).unwrap() == &'/' {
            index += 2;
            block += 2;
            is_inside_linear_comments = true;
            starting_index = index;
            continue;
        } else if character == &'/' && characters.get(index + 1).unwrap() == &'*' {
            index += 2;
            block += 2;
            is_inside_block_comments = true;
            starting_index = index;
            continue;
        } else if character == &'"' {
            index += 1;
            block += 1;
            is_inside_literal = true;
            starting_index = index;
            continue;
        }

        let delimiter = match *character {
            ',' => Some(Delimiter::Comma),
            ';' => Some(Delimiter::SemiColon),
            '(' => Some(Delimiter::OpeningParenthesis),
            ')' => Some(Delimiter::ClosingParenthesis),
            '[' => Some(Delimiter::OpeningBracket),
            ']' => Some(Delimiter::ClosingBracket),
            '{' => Some(Delimiter::OpeningCurlyBracket),
            '}' => Some(Delimiter::ClosingCurlyBracket),
            _ => None,
        };

        if let Some(delimiter) = delimiter {
            index += 1;
            block += 1;
            continue;
        }

        if *character == '+' {
            if characters[index + 1] == '+' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Increment),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::AdditionAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Addition),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '-' {
            if characters[index + 1] == '-' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Decrement),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::SubtractionAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Subtraction),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '*' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::MultiplicationAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Multiplication),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '/' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::DivisionAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Division),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '%' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::RemainderAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Remainder),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '=' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Equal),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Assignment),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '!' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::NotEqual),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::LogicalNegation),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '>' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BiggerOrEqual),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '>' {
                if characters[index + 2] == '=' {
                    tokens.push(Token {
                        line,
                        block,
                        token_type: TokenType::Operator(Operator::RightShiftAndAssignment),
                    });
                    index += 3;
                    block += 3;
                    continue;
                } else {
                    tokens.push(Token {
                        line,
                        block,
                        token_type: TokenType::Operator(Operator::RightShift),
                    });
                    index += 2;
                    block += 2;
                    continue;
                }
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Bigger),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '<' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::SmallerOrEqual),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '>' {
                if characters[index + 2] == '=' {
                    tokens.push(Token {
                        line,
                        block,
                        token_type: TokenType::Operator(Operator::LeftShiftAndAssignment),
                    });
                    index += 3;
                    block += 3;
                    continue;
                } else {
                    tokens.push(Token {
                        line,
                        block,
                        token_type: TokenType::Operator(Operator::LeftShift),
                    });
                    index += 2;
                    block += 2;
                    continue;
                }
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::Smaller),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '&' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BitwiseAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '&' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::LogicalAnd),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BitwiseAnd),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '|' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BitwiseOrAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else if characters[index + 1] == '|' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::LogicalOr),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BitwiseOr),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '^' {
            if characters[index + 1] == '=' {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::BitwiseExclusiveOrAndAssignment),
                });
                index += 2;
                block += 2;
                continue;
            } else {
                tokens.push(Token {
                    line,
                    block,
                    token_type: TokenType::Operator(Operator::ExclusiveOr),
                });
                index += 1;
                block += 1;
                continue;
            }
        } else if *character == '~' {
            tokens.push(Token {
                line,
                block,
                token_type: TokenType::Operator(Operator::BitwiseNegation),
            });
            index += 1;
            block += 1;
            continue;
        }

        // now find keyword
        // then find identifier

        let mut count = 0;
        let mut word: String = String::new();

        for character2 in characters[index..].iter() {
            if *character2 == '\n'
                || *character2 == ' '
                || delimiters.get(String::from(*character2).as_str()).is_some()
                || operators.get(String::from(*character2).as_str()).is_some()
                || operators
                    .get(
                        characters[(index + count)..(index + count + 1)]
                            .iter()
                            .collect::<String>()
                            .as_str(),
                    )
                    .is_some()
                || operators
                    .get(
                        characters[(index + count)..(index + count + 2)]
                            .iter()
                            .collect::<String>()
                            .as_str(),
                    )
                    .is_some()
            {
                break;
            }
            word.push(*character2);
            count += 1;
        }

        if let Some(keyword) = keywords.get(word.as_str()) {
            tokens.push(Token {
                line,
                block,
                token_type: TokenType::Keyword(*keyword),
            });
            index += count;
            block += count;
            continue;
        } else if identifer_regex.is_match(&word) {
            tokens.push(Token {
                line,
                block,
                token_type: TokenType::Identifier(word),
            });
            index += count;
            block += count;
            continue;
        } else {
            println!("error")
        }

        index += 1;
    }

    for token in tokens {
        println!("{}", token);
    }

    ExitCode::SUCCESS
}

fn help_text() -> String {
    "Name:
    c_scanner
Synopsis:
    c_scanner file_path
Description:
    Prints the tokens found in the file specified

    --help display this help text and exit"
        .to_string()
}

//TODO: detect numbers
//TODO: detect characters

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

pub struct Token {
    line: usize,
    block: usize,
    token_type: TokenType,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::Identifier(identifier) => {
                write!(
                    f,
                    "Identifier {} in line {} and block {}",
                    identifier, self.line, self.block
                )
            }
            TokenType::Comment(comment) => {
                write!(
                    f,
                    "Comment {} starting from line {} and block {}",
                    comment, self.line, self.block
                )
            }
            TokenType::Operator(operator) => {
                write!(
                    f,
                    "Operator {} found in line {} and block {}",
                    operator, self.line, self.block
                )
            }
            TokenType::Literal(literal) => {
                write!(
                    f,
                    "Literal {} found in line {} and block {}",
                    literal, self.line, self.block
                )
            }
            TokenType::Keyword(keyword) => {
                write!(
                    f,
                    "keyword {} found in line {} and block {}",
                    keyword, self.line, self.block
                )
            }
            TokenType::Number(number) => {
                write!(
                    f,
                    "Number {} found in line {} and block {}",
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
