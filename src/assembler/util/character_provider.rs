use std::{iter::Peekable, str::Chars};

use crate::assembler::token::Position;

pub struct CharacterProvider<'a> {
  chars: Peekable<Chars<'a>>,
  line: u32,
  column: u32,
}

impl<'a> CharacterProvider<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      chars: source.chars().peekable(),
      line: 1,
      column: 0,
    }
  }

  // pub fn current_location(&self) -> String {
  //     "NUL".into()
  // }

  pub fn current_position(&self) -> Position {
    Position {
      line: self.line,
      column: self.column,
    }
  }

  pub fn current_line(&self) -> u32 {
    self.line
  }

  pub fn current_column(&self) -> u32 {
    self.column
  }

  pub fn peek(&mut self) -> Option<&char> {
    self.chars.peek()
  }

  fn filtered_comment(&mut self, ch: &mut char) -> Option<bool> {
    if let Some(peek) = self.chars.peek() {
      if *peek == '/' {
        while *ch != '\n' {
          self.column += 1;
          *ch = self.chars.next()?;
        }
        self.line += 1;
        self.column = 0;

        return Some(true);
      }
    }

    Some(false)
  }

  pub fn next_with_spaces(&mut self) -> Option<char> {
    loop {
      let mut ch = self.chars.next()?;
      self.column += 1;
      match ch {
        whitespace if whitespace.is_whitespace() => {
          if ch == '\n' {
            self.line += 1;
            self.column = 0;
          } else {
            break Some(ch);
          }
        }
        '/' => {
          if !self.filtered_comment(&mut ch)? {
            break Some(ch);
          }
        }
        _ => break Some(ch),
      }
    }
  }
}

impl Iterator for CharacterProvider<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let mut ch = self.chars.next()?;
      self.column += 1;
      match ch {
        whitespace if whitespace.is_whitespace() => {
          if ch == '\n' {
            self.line += 1;
            self.column = 0;
          }
        }
        '/' => {
          if !self.filtered_comment(&mut ch)? {
            break Some(ch);
          }
        }
        _ => break Some(ch),
      }
    }
  }
}
