use super::token::*;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum LexError {
    UnterminatedString,
    InvalidCharLiteral,
    UnexpectedCharacter(char),
    InvalidEscapeSequence(char),
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            chars: input.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.advance() {
            Some(c) => match c {
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::Equals
                    }
                    _ => Token::Assign,
                },
                '+' => match self.peek() {
                    Some('+') => {
                        self.advance();
                        Token::Increment
                    }
                    Some('=') => {
                        self.advance();
                        Token::PlusEquals
                    }
                    _ => Token::Plus,
                },
                '-' => match self.peek() {
                    Some('-') => {
                        self.advance();
                        Token::Decrement
                    }
                    Some('=') => {
                        self.advance();
                        Token::MinusEquals
                    }
                    Some('>') => {
                        self.advance();
                        Token::Arrow
                    }
                    _ => Token::Minus,
                },
                '*' => match self.peek() {
                    Some('*') => {
                        self.advance();
                        Token::Power
                    }
                    Some('=') => {
                        self.advance();
                        Token::TimesEquals
                    }
                    _ => Token::Multiply,
                },
                '/' => match self.peek() {
                    Some('/') => {
                        self.advance();
                        self.skip_line_comment();
                        self.next_token()
                    }
                    Some('=') => {
                        self.advance();
                        Token::DivideEquals
                    }
                    _ => Token::Divide,
                },
                '%' => Token::Modulo,
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::NotEquals
                    }
                    _ => Token::Bang,
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::LessThanEquals
                    }
                    _ => Token::LessThan,
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::GreaterThanEquals
                    }
                    _ => Token::GreaterThan,
                },
                '&' => match self.advance() {
                    Some('&') => Token::And,
                    _ => panic!("Unexpected character after '&'"), // Handle error properly
                },
                '|' => match self.advance() {
                    Some('|') => Token::Or,
                    _ => panic!("Unexpected character after '|'"), // Handle error properly
                },
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,
                ',' => Token::Comma,
                '"' => self.tokenize_string(),
                '\'' => self.tokenize_char(),
                c if c.is_digit(10) => self.tokenize_number(c),
                c if c.is_alphabetic() => self.tokenize_identifier(c),
                _ => Token::EOF,
            },
            None => Token::EOF,
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if *c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn tokenize_string(&mut self) -> Token {
        let mut string_value = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                return Token::LiteralString(string_value);
            }
            string_value.push(c);
        }
        panic!("Unterminated string literal");
    }

    fn tokenize_char(&mut self) -> Token {
        let c = self.advance().expect("Expected a character");
        if self.advance() != Some('\'') {
            panic!("Invalid char literal");
        }
        Token::LiteralChar(c)
    }

    fn tokenize_number(&mut self, first_digit: char) -> Token {
        let mut number_string = String::new();
        number_string.push(first_digit);
        while let Some(c) = self.peek() {
            if c.is_digit(10) || *c == '.' {
                number_string.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        Token::LiteralNumber(number_string)
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c.clone() == '_' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "elif" => Token::Elif,
            "for" => Token::For,
            "while" => Token::While,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "bool" => Token::Bool,
            "string" => Token::String,
            "char" => Token::Char,
            "u8" => Token::U8,
            "i8" => Token::I8,
            "u16" => Token::U16,
            "i16" => Token::I16,
            "f16" => Token::F16,
            "u32" => Token::U32,
            "i32" => Token::I32,
            "f32" => Token::F32,
            "u64" => Token::U64,
            "i64" => Token::I64,
            "f64" => Token::F64,
            _ => Token::Identifier(identifier),
        }
    }
}