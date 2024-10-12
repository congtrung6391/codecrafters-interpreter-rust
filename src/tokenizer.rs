use crate::token::{
    TokenType,
    Token,
};

pub struct Tokenizer {
    tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        return Tokenizer { tokens: Vec::new() };
    }

    fn is_digit(char: char) -> bool {
        if char.to_digit(10) != None {
            return true;
        }
        return false;
    }

    fn is_alpha(char: char) -> bool {
        if char >= 'a' && char <= 'z' {
            return true;
        }

        if char >= 'A' && char <= 'Z' {
            return true;
        }

        if char == '_' {
            return true;
        }

        return false;
    }

    fn is_alpha_numberic(char: char) -> bool {
        return Self::is_alpha(char) || Self::is_digit(char);
    }

    fn get_reserverd_word_token_type(char: &str) -> TokenType {
        match char {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<String>) {
        let token = Token::new(
            token_type,
            lexeme,
            literal,
        );
        let _ = &self.tokens.push(token);
    }

    fn lexer_error(line: i32, message: String) {
        eprintln!("[line {}] Error: {}", line, message)
    }

    pub fn scan(&mut self, file_contents: String) -> i32 {
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
                '(' => Self::add_token(self, TokenType::LEFT_PAREN, String::from(char), None),
                ')' => Self::add_token(self, TokenType::RIGHT_PAREN, String::from(char), None),
                '{' => Self::add_token(self, TokenType::LEFT_BRACE, String::from(char), None),
                '}' => Self::add_token(self, TokenType::RIGHT_BRACE, String::from(char), None),
                ',' => Self::add_token(self, TokenType::COMMA, String::from(char), None),
                ';' => Self::add_token(self, TokenType::SEMICOLON, String::from(char), None),
                '+' => Self::add_token(self, TokenType::PLUS, String::from(char), None),
                '-' => Self::add_token(self, TokenType::MINUS, String::from(char), None),
                '*' => Self::add_token(self, TokenType::STAR, String::from(char), None),
                '.' => Self::add_token(self, TokenType::DOT, String::from(char), None),
                '=' => {
                    if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                        Self::add_token(
                            self,
                            TokenType::EQUAL_EQUAL,
                            String::from(char.to_string() + &char_at(index + 1).to_string()),
                            None,
                        );
                        index += 1;
                    } else {
                        Self::add_token(self, TokenType::EQUAL, String::from(char), None);
                    }
                }
                '<' => {
                    if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                        Self::add_token(
                            self,
                            TokenType::LESS_EQUAL,
                            String::from(char.to_string() + &char_at(index + 1).to_string()),
                            None,
                        );
                        index += 1;
                    } else {
                        Self::add_token(self, TokenType::LESS, String::from(char), None);
                    }
                }
                '>' => {
                    if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                        Self::add_token(
                            self,
                            TokenType::GREATER_EQUAL,
                            String::from(char.to_string() + &char_at(index + 1).to_string()),
                            None,
                        );
                        index += 1;
                    } else {
                        Self::add_token(self, TokenType::GREATER, String::from(char), None);
                    }
                }
                '!' => {
                    if index + 1 < file_contents_len && char_at(index + 1) == '=' {
                        Self::add_token(
                            self,
                            TokenType::BANG_EQUAL,
                            String::from(char.to_string() + &char_at(index + 1).to_string()),
                            None,
                        );
                        index += 1;
                    } else {
                        Self::add_token(self, TokenType::BANG, String::from(char), None);
                    }
                }
                '/' => {
                    if index + 1 < file_contents_len && char_at(index + 1) == '/' {
                        while index + 1 < file_contents_len && char_at(index + 1) != '\n' {
                            index += 1;
                        }
                    } else {
                        Self::add_token(self, TokenType::SLASH, String::from(char), None);
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
                        Self::add_token(self, TokenType::STRING, lexeme_format, Some(literal));
                    } else {
                        Self::lexer_error(line, String::from("Unterminated string."));
                        result = 65;
                    }
                }
                ' ' => {}
                '\t' => {}
                '\r' => {}
                '\0' => {}
                '\n' => {
                    line += 1;
                }
                _ => {
                    if Self::is_digit(char) {
                        let mut literal = String::from("");
                        let mut is_float = false;
                        while index < file_contents_len
                            && (Self::is_digit(char_at(index)) || char_at(index) == '.')
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

                        Self::add_token(self, TokenType::NUMBER, literal.clone(), Some(literal));
                    } else if Self::is_alpha(char) {
                        let mut literal = String::from("");
                        while index < file_contents_len && Self::is_alpha_numberic(char_at(index)) {
                            literal = literal + &char_at(index).to_string();
                            index += 1;
                        }
                        index -= 1;

                        Self::add_token(
                            self,
                            Self::get_reserverd_word_token_type(literal.clone().as_str()),
                            literal.clone(),
                            None,
                        );
                    } else {
                        Self::lexer_error(
                            line,
                            String::from(format!("Unexpected character: {}", char)),
                        );
                        result = 65;
                    }
                }
            }

            index += 1;
        }

        return result;
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        return self.tokens.clone();
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.clone() {
            token.to_string();
        }
        println!("EOF  null");
    }
}


