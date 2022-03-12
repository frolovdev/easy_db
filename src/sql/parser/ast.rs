

use super::super::types::DataType;
use crate::error::{EasyDbResult, EasyDbError};

use super::lexer::{Lexer, Token, Keyword};
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

    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
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
        Parser { lexer: Lexer::new(query).peekable() }
    } 

    pub fn parse(&mut self) -> EasyDbResult<Statement> {

    } 

    /// Get the next lexer token, or throws an error if none is found.
    fn next(&mut self) -> EasyDbResult<Token> {
        self.lexer.next().unwrap_or_else(|| Err(EasyDbError::Parse("Unexpected end of input".into())))
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
        match self.next()? {}
    }
}