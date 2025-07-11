use super::token::*;

#[derive(Debug)]
struct Tokenizer {
    text: String,
    pos: usize,
    previous_token: Option<Token>,
    current_char: Option<char>,
}

impl Tokenizer {
    fn new(text: String) -> Self {
        let mut tokenizer = Tokenizer {
            text,
            pos: 0,
            previous_token: None,
            current_char: None,
        };
        tokenizer.advance();
        tokenizer
    }

    fn advance(&mut self) {
        if self.pos < self.text.len() {
            self.current_char = self.text.chars().nth(self.pos);
            self.pos += 1;
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut token_str = String::new();
        if self.current_char == '"' {
            self.advance();

            let mut char_escaped: bool = false;

            // handle strings including escape characters
            while let Some(c) = self.current_char {
                if char_escaped {
                    if c == 'n' {
                        token_str.push('\n');
                    } else if c == 't' {
                        token_str.push('\t');
                    } else {
                        token_str.push(c.clone());
                    }
                    char_escaped = false;
                    continue;
                }

                if c == '"' {
                    break;
                }

                if c == '\\' {
                    char_escaped = true;
                    continue;
                }

                token_str.push(c.c.lone());
            }

            token_str
        } else {
            while let Some(c) = self.current_char {
                if c.is_whitespace() {
                    break;
                }

                token_str.push(c.clone());

                self.advance();

                // brackets aren't surrounded by whitespace
                if c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' {
                    break;
                }
            }
        }

        match token_str {
            Some("fn") => Token::Fn,
            Some("if") => Token::If,
            Some("else") => Token::Else,
            Some("elif") => Token::Elif,
            Some("for") => Token::For,
            Some("while") => Token::While,
            Some("return") => Token::Return,
            Some("true") => Token::True,
            Some("false") => Token::False,

            Some("bool") => Token::Bool,
            Some("string") => Token::String,
            Some("char") => Token::Char,
            Some("u8") => Token::U8,
            Some("i8") => Token::I8,
            Some("u16") => Token::U16,
            Some("i16") => Token::I16,
            Some("f16") => Token::F16,
            Some("u32") => Token::U32,
            Some("i32") => Token::I32,
            Some("f32") => Token::F32,
            Some("u64") => Token::U64,
            Some("i64") => Token::I64,
            Some("f64") => Token::F64,

            Some("=") => Token::Assign,
            Some("+") => Token::Plus,
            Some("-") => Token::Minus,
            Some("*") => Token::Multiply,
            Some("/") => Token::Divide,
            Some("%") => Token::Modulo,
            Some("**") => Token::Power,
            Some("!") => Token::Bang,

            Some("==") => Token::Equals,
            Some("!=") => Token::NotEquals,
            Some("<") => Token::LessThan,
            Some(">") => Token::GreaterThan,
            Some("<=") => Token::LessThanEquals,
            Some(">=") => Token::GreaterThanEquals,

            Some("&&") => Token::And,
            Some("||") => Token::Or

            Some("++") => Token::Increment,
            Some("+=") => Token::PlusEquals,
            Some("--") => Token::Decrement,
            Some("-=") => Token::MinusEquals,
            Some("*=") => Token::TimesEquals,
            Some("/=") => Token::DivideEquals,

            Some("(") => Token::OpenParen,
            Some(")") => Token::CloseParen,
            Some("{") => Token::OpenBrace,
            Some("}") => Token::CloseBrace,
            Some("[") => Token::OpenBracket,
            Some("]") => Token::CloseBracket
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.current_char != None {
            let next_token: Token = self.get_next_token();
            tokens.push(Token);
            self.previous_token = Token;
        }
    }
}
