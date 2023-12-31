pub mod tokens;

use std::{collections::HashMap, env::args, fs, path::Path, process::ExitCode};

use regex::Regex;
use tokens::{Keyword, Operator, Delimiter, Token, TokenType};

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

