use std::{iter::Peekable, slice::Iter};

use crate::assembler::token::{Position, Token};

#[derive(Debug, Clone)]
pub enum Next<T> {
  Token(T),
  EndOfFile { position: Position },
  EndOfStream { position: Position },
}

impl<T: Clone> Next<&T> {
  pub fn cloned(&self) -> Next<T> {
    match self {
      &Next::Token(token) => Next::Token(token.clone()),
      Next::EndOfFile { position } => Next::EndOfFile {
        position: position.clone(),
      },
      Next::EndOfStream { position } => Next::EndOfStream {
        position: position.clone(),
      },
    }
  }
}

pub struct TokenProvider<'a> {
  previous_valid_token: Token,
  tokens: Peekable<Iter<'a, Token>>,
  last_line: u32,
  last_column: u32,
}

impl<'a> TokenProvider<'a> {
  pub fn new(tokens: &'a [Token]) -> Self {
    Self {
      previous_valid_token: Token::EndOfFile {
        position: Position::default(),
      },
      tokens: tokens.iter().peekable(),
      last_line: 0,
      last_column: 0,
    }
  }

  pub fn peek(&mut self) -> Next<&Token> {
    match self.tokens.peek() {
      Some(token) => match token {
        Token::EndOfFile { position } => {
          self.last_line = position.line;
          self.last_column = position.column;
          Next::EndOfFile {
            position: position.clone(),
          }
        }
        &t => Next::Token(t),
      },
      None => Next::EndOfStream {
        position: Position::new(self.last_line, self.last_column),
      },
    }
  }

  pub fn previous_valid(&self) -> &Token {
    &self.previous_valid_token
  }

  pub fn next(&mut self) -> Next<&Token> {
    match self.tokens.next() {
      Some(token) => match token {
        Token::EndOfFile { position } => Next::EndOfFile {
          position: position.clone(),
        },
        t => {
          self.previous_valid_token = t.clone();
          Next::Token(t)
        }
      },
      None => Next::EndOfStream {
        position: Position::new(self.last_line, self.last_column),
      },
    }
  }
}

// impl<'a> Iterator for TokenProvider<'a> {
//     type Item = &'a Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.tokens.next() {
//             Some(token) => match token {
//                 &Token::EndOfFile { .. } => None,
//                 t => Some(t),
//             },
//             None => None,
//         }
//     }
// }
