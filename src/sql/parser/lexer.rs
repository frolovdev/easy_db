use crate::error::{EasyDbError, EasyDbResult};

use std::iter::Peekable;
use std::str::Chars;

// A lexer token
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    String(String),
    Ident(String),
    Keyword(Keyword),
    Period,
    Equal,
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Percent,
    Exclamation,
    Question,
    OpenParen,
    CloseParen,
    Comma,
    Semicolon,
    GreaterThanOrEqual,
    LessThanOrEqual,
    LessOrGreaterThan,
    NotEqual,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Token::Number(n) => n,
            Token::String(s) => s,
            Token::Ident(s) => s,
            Token::Keyword(k) => k.to_str(),
            Token::Period => ".",
            Token::Equal => "=",
            Token::GreaterThan => ">",
            Token::GreaterThanOrEqual => ">=",
            Token::LessThan => "<",
            Token::LessThanOrEqual => "<=",
            Token::LessOrGreaterThan => "<>",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Caret => "^",
            Token::Percent => "%",
            Token::Exclamation => "!",
            Token::NotEqual => "!=",
            Token::Question => "?",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::Comma => ",",
            Token::Semicolon => ";",
        })
    }
}

/*
same as

let my_str = "hello"; // str

let my_string = String::from(my_str); // string
*/
impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

// supported keywords
#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    And,
    Create,
    Drop,
    Table,
    Bool,
    Boolean,
    Char,
    Double,
    Float,
    Int,
    Integer,
    String,
    Text,
    Varchar,
    Primary,
    Key,
    Null,
    Not,
    Default,
    Unique,
    Index,
    References,
}

impl Keyword {}

impl Keyword {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(ident: &str) -> Option<Self> {
        Some(match ident.to_uppercase().as_ref() {
            "AND" => Self::And,
            _ => return None,
        })
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::And => "AND",
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

/// just an iterator
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = EasyDbResult<Token>;

    fn next(&mut self) -> Option<EasyDbResult<Token>> {
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self
                .iter
                .peek()
                .map(|c| Err(EasyDbError::Parse(format!("Unexpected character {}", c)))),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            iter: input.chars().peekable(),
        }
    }

    /// Scans the input for the next token if any, ignoring leading whitespace
    fn scan(&mut self) -> EasyDbResult<Option<Token>> {
        self.skip_whitespace();
        match self.iter.peek() {
            // Some('\'') => self.scan_string(),
            // Some('"') => self.scan_ident_quoted(),
            Some(c) if c.is_digit(10) => Ok(self.scan_number()),
            // Some(c) if c.is_alphabetic() => Ok(self.scan_ident()),
            Some(_) => Ok(self.scan_symbol()),
            None => Ok(None),
        }
    }

    fn skip_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(|&c| predicate(*c))?;
        self.iter.next()
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value = String::new();

        while let Some(c) = self.next_if(&predicate) {
            value.push(c)
        }

        Some(value).filter(|v| !v.is_empty())
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_digit(10))?;

        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);

            while let Some(dec) = self.next_if(|c| c.is_digit(10)) {
                num.push(dec)
            }
        }

        if let Some(exp) = self.next_if(|c| c == 'e' || c == 'E') {
            num.push(exp);
            if let Some(sign) = self.next_if(|c| c == '+' || c == '-') {
                num.push(sign)
            }
            while let Some(c) = self.next_if(|c| c.is_digit(10)) {
                num.push(c)
            }
        }

        Some(Token::Number(num))
    }

    /// Grabs the next single-character token if the tokenizer function returns one
    fn next_if_token<F: Fn(char) -> Option<Token>>(&mut self, tokenizer: F) -> Option<Token> {
        let token = self.iter.peek().and_then(|&c| tokenizer(c))?;
        self.iter.next();
        Some(token)
    }

    fn scan_symbol(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '.' => Some(Token::Period),
            '=' => Some(Token::Equal),
            '>' => Some(Token::GreaterThan),
            '<' => Some(Token::LessThan),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '^' => Some(Token::Caret),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            '?' => Some(Token::Question),
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            _ => None,
        })
        .map(|token| match token {
            Token::Exclamation => {
                if self.next_if(|c| c == '=').is_some() {
                    Token::NotEqual
                } else {
                    token
                }
            }
            Token::LessThan => {
                if self.next_if(|c| c == '>').is_some() {
                    Token::LessOrGreaterThan
                } else if self.next_if(|c| c == '=').is_some() {
                    Token::LessThanOrEqual
                } else {
                    token
                }
            }
            Token::GreaterThan => {
                if self.next_if(|c| c == '=').is_some() {
                    Token::GreaterThanOrEqual
                } else {
                    token
                }
            }
            _ => token,
        })
    }
}
