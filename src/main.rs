use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

#[derive(Debug, PartialEq)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

// #[derive(Debug)]
// #[derive(Clone)]
// enum Literal {
//     Text(String),
//     Number(i32),
// }

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
}

impl Token {
    pub fn to_string(&self) {
        let copied_literal = self.literal.clone();
        if self.token_type == TokenType::NUMBER {
            let literal_number = copied_literal
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            let formatted = if literal_number.fract() == 0.0 {
                format!("{:.1}", literal_number) // Ensure at least one decimal place
            } else {
                literal_number.to_string() // Use the default string representation
            };
            println!("{:#?} {} {}", self.token_type, self.lexeme, formatted)
        } else {
            println!(
                "{:#?} {} {}",
                self.token_type,
                self.lexeme,
                copied_literal.unwrap_or(String::from("null"))
            );
        }
    }
}

fn add_token(token_type: TokenType, lexeme: String, literal: Option<String>) {
    let token = Token {
        token_type,
        lexeme,
        literal,
    };
    token.to_string();
}

fn lexer_error(line: i32, message: String) {
    eprintln!("[line {}] Error: {}", line, message)
}

fn is_digit(char: char) -> bool {
    if char.to_digit(10) != None {
        return true;
    }
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let result = scanner(file_contents);
                exit(result);
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn scanner(file_contents: String) -> i32 {
    let mut result = 0;
    let mut line = 1;

    let mut index: usize = 0;
    let file_contents_len = file_contents.len();

    let char_at = |idx: usize| {
        return file_contents.chars().nth(idx).unwrap_or_default();
    };

    while index < file_contents_len {
        let char = file_contents.chars().nth(index).unwrap_or_default();

        match char {
            '(' => add_token(TokenType::LEFT_PAREN, String::from(char), None),
            ')' => add_token(TokenType::RIGHT_PAREN, String::from(char), None),
            '{' => add_token(TokenType::LEFT_BRACE, String::from(char), None),
            '}' => add_token(TokenType::RIGHT_BRACE, String::from(char), None),
            ',' => add_token(TokenType::COMMA, String::from(char), None),
            ';' => add_token(TokenType::SEMICOLON, String::from(char), None),
            '+' => add_token(TokenType::PLUS, String::from(char), None),
            '-' => add_token(TokenType::MINUS, String::from(char), None),
            '*' => add_token(TokenType::STAR, String::from(char), None),
            '.' => add_token(TokenType::DOT, String::from(char), None),
            '=' => {
                if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                    add_token(
                        TokenType::EQUAL_EQUAL,
                        String::from(char.to_string() + &char_at(index + 1).to_string()),
                        None,
                    );
                    index += 1;
                } else {
                    add_token(TokenType::EQUAL, String::from(char), None);
                }
            }
            '<' => {
                if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                    add_token(
                        TokenType::LESS_EQUAL,
                        String::from(char.to_string() + &char_at(index + 1).to_string()),
                        None,
                    );
                    index += 1;
                } else {
                    add_token(TokenType::LESS, String::from(char), None);
                }
            }
            '>' => {
                if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                    add_token(
                        TokenType::GREATER_EQUAL,
                        String::from(char.to_string() + &char_at(index + 1).to_string()),
                        None,
                    );
                    index += 1;
                } else {
                    add_token(TokenType::GREATER, String::from(char), None);
                }
            }
            '!' => {
                if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                    add_token(
                        TokenType::BANG_EQUAL,
                        String::from(char.to_string() + &char_at(index + 1).to_string()),
                        None,
                    );
                    index += 1;
                } else {
                    add_token(TokenType::BANG, String::from(char), None);
                }
            }
            '/' => {
                if index + 1 < file_contents_len && char_at(index + 1) == '/' {
                    while index + 1 < file_contents_len && char_at(index + 1) != '\n' {
                        index += 1;
                    }
                } else {
                    add_token(TokenType::SLASH, String::from(char), None);
                }
            }
            '"' => {
                let mut lexeme = String::from("");
                index += 1;
                while index < file_contents_len && char_at(index) != '"' {
                    lexeme = lexeme + &char_at(index).to_string();
                    index += 1;
                }

                if index < file_contents_len {
                    let literal: String = lexeme.clone();
                    let lexeme_format = format!("\"{}\"", lexeme.clone());
                    add_token(TokenType::STRING, lexeme_format, Some(literal));
                } else {
                    lexer_error(line, String::from("Unterminated string."));
                    result = 65;
                }
            }
            ' ' => {}
            '\t' => {}
            '\r' => {}
            '\n' => {
                line += 1;
            }
            _ => {
                if is_digit(char) {
                    let mut literal = String::from("");
                    let mut is_float = false;
                    while index < file_contents_len
                        && (is_digit(char_at(index)) || char_at(index) == '.')
                    {
                        if char_at(index) == '.' {
                            if is_float == true {
                                break;
                            }
                            is_float = true;
                        }

                        literal = literal + &char_at(index).to_string();
                        index += 1;
                    }
                    index -= 1;

                    add_token(TokenType::NUMBER, literal.clone(), Some(literal));
                } else {
                    lexer_error(
                        line,
                        String::from(format!("Unexpected character: {}", char)),
                    );
                    result = 65;
                }
            }
        }

        index += 1;
    }

    println!("EOF  null");
    return result;
}
