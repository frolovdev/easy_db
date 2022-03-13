use super::super::types::DataType;
use crate::error::{EasyDbError, EasyDbResult};

use super::lexer::{Keyword, Lexer, Token};
use std::collections::BTreeMap;
use std::mem::replace;

/// Statements
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    // Begin {
    //     readonly: bool,
    //     version: Option<u64>,
    // },
    // Commit,
    // Rollback,
    // Explain(Box<Statement>),
    CreateTable { name: String, columns: Vec<Column> },
    DropTable(String),
    // Delete {
    //     table: String,
    //     r#where: Option<Expression>,
    // },
    // Insert {
    //     table: String,
    //     columns: Option<Vec<String>>,
    //     values: Vec<Vec<Expression>>,
    // },
    // Update {
    //     table: String,
    //     set: BTreeMap<String, Expression>,
    //     r#where: Option<Expression>,
    // },

    // Select {
    //     select: Vec<(Expression, Option<String>)>,
    //     from: Vec<FromItem>,
    //     r#where: Option<Expression>,
    //     group_by: Vec<Expression>,
    //     having: Option<Expression>,
    //     order: Vec<(Expression, Order)>,
    //     offset: Option<Expression>,
    //     limit: Option<Expression>,
    // },
}

/// A FROM item
// #[derive(Clone, Debug, PartialEq)]
// pub enum FromItem {
//     Table {
//         name: String,
//         alias: Option<String>,
//     },
//     Join {
//         left: Box<FromItem>,
//         right: Box<FromItem>,
//         r#type: JoinType,
//         predicate: Option<Expression>,
//     },
// }

/// A JOIN type
// #[derive(Clone, Debug, PartialEq)]
// pub enum JoinType {
//     Cross,
//     Inner,
//     Left,
//     Right,
// }

/// A column
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
    pub primary_key: bool,
    pub nullable: Option<bool>,
    // TODO: implement expressions
    // pub default: Option<Expression>,
    pub unique: bool,
    pub index: bool,
    pub references: Option<String>,
}

pub struct Parser<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(query: &str) -> Parser {
        Parser {
            lexer: Lexer::new(query).peekable(),
        }
    }

    pub fn parse(&mut self) -> EasyDbResult<Statement> {
        let statement = self.parse_statement()?;
        self.next_if_token(Token::Semicolon);
        self.next_expect(None)?;
        Ok(statement)
    }

    /// Get the next lexer token, or throws an error if none is found.
    fn next(&mut self) -> EasyDbResult<Token> {
        self.lexer
            .next()
            .unwrap_or_else(|| Err(EasyDbError::Parse("Unexpected end of input".into())))
    }

    /// Grabs the next lexer token if it satisfies the predicate function
    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Option<Token> {
        self.peek().unwrap_or(None).filter(|t| predicate(t))?;
        self.next().ok()
    }

    /// Grabs the next lexer token if it is a given token
    fn next_if_token(&mut self, token: Token) -> Option<Token> {
        self.next_if(|t| t == &token)
    }

    /// Grabs the next lexer token if it is a keyword
    fn next_if_keyword(&mut self) -> Option<Token> {
        self.next_if(|t| matches!(t, Token::Keyword(_)))
    }

    /// Grabs the next lexer token, and returns it if it was expected or
    /// otherwise throws an error.
    fn next_expect(&mut self, expect: Option<Token>) -> EasyDbResult<Option<Token>> {
        if let Some(t) = expect {
            let token = self.next()?;
            if token == t {
                Ok(Some(token))
            } else {
                Err(EasyDbError::Parse(format!(
                    "Expected token {}, found {}",
                    t, token
                )))
            }
        } else if let Some(token) = self.peek()? {
            Err(EasyDbError::Parse(format!("Unexpected token {}", token)))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> EasyDbResult<Option<Token>> {
        self.lexer.peek().cloned().transpose()
    }

    fn parse_statement(&mut self) -> EasyDbResult<Statement> {
        match self.peek()? {
            Some(Token::Keyword(Keyword::Create)) => self.parse_ddl(),
            Some(token) => Err(EasyDbError::Parse(format!("Unexpected token {}", token))),
            None => Err(EasyDbError::Parse("Unexpected end of input".into())),
        }
    }

    fn parse_ddl(&mut self) -> EasyDbResult<Statement> {
        match self.next()? {
            Token::Keyword(Keyword::Create) => match self.next()? {
                Token::Keyword(Keyword::Table) => self.parse_ddl_create_table(),
                token => Err(EasyDbError::Parse(format!("Unexpected token {}", token))),
            },
            Token::Keyword(Keyword::Drop) => match self.next()? {
                Token::Keyword(Keyword::Table) => self.parse_ddl_drop_table(),
                token => Err(EasyDbError::Parse(format!("Unexpected token {}", token))),
            },
            token => Err(EasyDbError::Parse(format!("Unexpected token {}", token))),
        }
    }

    /// Grabs the next identifier, or errors if not found
    fn next_ident(&mut self) -> EasyDbResult<String> {
        match self.next()? {
            Token::Ident(ident) => Ok(ident),
            token => Err(EasyDbError::Parse(format!(
                "Expected identifier, got {}",
                token
            ))),
        }
    }

    /// Parses a CREATE TABLE DDL statement. The CREATE TABLE prefix has
    /// already been consumed.
    fn parse_ddl_create_table(&mut self) -> EasyDbResult<Statement> {
        let name = self.next_ident()?;
        self.next_expect(Some(Token::OpenParen))?;

        let mut columns = Vec::new();

        loop {
            columns.push(self.parse_ddl_column()?);
            if self.next_if_token(Token::Comma).is_none() {
                break;
            }
        }

        self.next_expect(Some(Token::CloseParen))?;
        Ok(Statement::CreateTable { name, columns })
    }

    fn parse_ddl_column(&mut self) -> EasyDbResult<Column> {
        let mut column = Column {
            name: self.next_ident()?,
            datatype: match self.next()? {
                Token::Keyword(Keyword::Bool) => DataType::Boolean,
                Token::Keyword(Keyword::Boolean) => DataType::Boolean,
                Token::Keyword(Keyword::Char) => DataType::String,
                Token::Keyword(Keyword::Double) => DataType::Float,
                Token::Keyword(Keyword::Float) => DataType::Float,
                Token::Keyword(Keyword::Int) => DataType::Integer,
                Token::Keyword(Keyword::Integer) => DataType::Integer,
                Token::Keyword(Keyword::String) => DataType::String,
                Token::Keyword(Keyword::Text) => DataType::String,
                Token::Keyword(Keyword::Varchar) => DataType::String,
                token => return Err(EasyDbError::Parse(format!("Unexpected token {}", token))),
            },
            primary_key: false,
            nullable: None,
            // default: None,
            unique: false,
            index: false,
            references: None,
        };

        while let Some(Token::Keyword(keyword)) = self.next_if_keyword() {
            match keyword {
                Keyword::Primary => {
                    self.next_expect(Some(Keyword::Key.into()))?;
                    column.primary_key = true;
                }
                Keyword::Null => {
                    if let Some(false) = column.nullable {
                        return Err(EasyDbError::Value(format!(
                            "Column {} can't be both not nullable and nullable",
                            column.name
                        )));
                    }
                    column.nullable = Some(true)
                }
                // Keyword::Default => column.default = Some(self.parse_expression(0)?),
                Keyword::Unique => column.unique = true,
                Keyword::Index => column.index = true,
                Keyword::References => column.references = Some(self.next_ident()?),
                Keyword::Not => {
                    self.next_expect(Some(Keyword::Null.into()))?;
                    if let Some(true) = column.nullable {
                        return Err(EasyDbError::Value(format!(
                            "Column {} can't be both not nullable and nullable",
                            column.name
                        )));
                    }
                    column.nullable = Some(false)
                }
                keyword => {
                    return Err(EasyDbError::Parse(format!(
                        "Unexpected keyword {}",
                        keyword
                    )))
                }
            }
        }

        Ok(column)
    }

    /// Parses a DROP TABLE DDL statement. The DROP TABLE prefix has
    /// already been consumed.
    fn parse_ddl_drop_table(&mut self) -> EasyDbResult<Statement> {
        Ok(Statement::DropTable(self.next_ident()?))
    }
}
