use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

#[derive(Debug)]
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

struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn to_string(&self) {
        println!("{:#?} {} null", self.token_type, self.lexeme);
    }
}

fn add_token(token_type: TokenType, lexeme: String) {
    let token = Token { token_type, lexeme };
    token.to_string();
}

fn lexer_error(line: i32, token: String) {
    eprintln!("[line {}] Error: Unexpected character: {}", line, token)
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
    let line = 1;

    let mut prev_char_opr: Option<char> = None;

    for char in file_contents.chars() {
        if prev_char_opr != None {
           if char == '=' {
               match prev_char_opr {
                   Some('=') => add_token(TokenType::EQUAL_EQUAL, String::from(prev_char_opr.unwrap_or_default().to_string() + &char.to_string())),
                   Some('<') => add_token(TokenType::LESS_EQUAL, String::from(prev_char_opr.unwrap_or_default().to_string() + &char.to_string())),
                   Some('>') => add_token(TokenType::GREATER_EQUAL, String::from(prev_char_opr.unwrap_or_default().to_string() + &char.to_string())),
                   Some('!') => add_token(TokenType::BANG_EQUAL, String::from(prev_char_opr.unwrap_or_default().to_string() + &char.to_string())),
                   _ => {}
               }
               prev_char_opr = None;
               continue
           } else {
               match prev_char_opr {
                   Some('=') => add_token(TokenType::EQUAL, String::from(prev_char_opr.unwrap_or_default())),
                   Some('<') => add_token(TokenType::LESS, String::from(prev_char_opr.unwrap_or_default())),
                   Some('>') => add_token(TokenType::GREATER, String::from(prev_char_opr.unwrap_or_default())),
                   Some('!') => add_token(TokenType::BANG_EQUAL, String::from(prev_char_opr.unwrap_or_default())),
                   _ => {}
               }
           } 
           prev_char_opr = None;
        }

        match char {
            '(' => add_token(TokenType::LEFT_PAREN, String::from(char)),
            ')' => add_token(TokenType::RIGHT_PAREN, String::from(char)),
            '{' => add_token(TokenType::LEFT_BRACE, String::from(char)),
            '}' => add_token(TokenType::RIGHT_BRACE, String::from(char)),
            ',' => add_token(TokenType::COMMA, String::from(char)),
            ';' => add_token(TokenType::SEMICOLON, String::from(char)),
            '+' => add_token(TokenType::PLUS, String::from(char)),
            '-' => add_token(TokenType::MINUS, String::from(char)),
            '*' => add_token(TokenType::STAR, String::from(char)),
            '.' => add_token(TokenType::DOT, String::from(char)),
            '=' => prev_char_opr = Some(char),
            '>' => prev_char_opr = Some(char),
            '<' => prev_char_opr = Some(char),
            '!' => prev_char_opr = Some(char),
            '\n' => {},
            _ => {
                lexer_error(line, String::from(char));
                result = 65;
            }
        }
    }

    println!("EOF  null");
    return result;
}
