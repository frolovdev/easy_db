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
}


impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.write_str(match self {
          Token::Number(n) => n,
          Token::String(s) => s,
          Token::Ident(s) => s,
          Token::Keyword(k) => k.to_str(),
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
}

impl Keyword { }


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
      Self::And => "AND"
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

// impl<'a> Iterator for Lexer<'a> {
//   type Item = Result<Token>;

//   fn next(&mut self) -> Option<Result<Token>> {
//       match self.scan() {
//           Ok(Some(token)) => Some(Ok(token)),
//           Ok(None) => self.iter.peek().map(|c| Err(Error::Parse(format!("Unexpected character {}", c)))),
//           Err(err) => Some(Err(err)),
//       }
//   }
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}