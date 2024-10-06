pub const RESERVED_WORDS: [&str; 16] = [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
];

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>) -> Token {
        Token {
            token_type,
            lexeme,
            literal
        }
    }

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


