use std::fmt::{Display, Formatter};

use strum::EnumDiscriminants;

use super::{keyword::Keyword, literal::Literal, symbol::Symbol};

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
pub enum Token {
  Symbol(SymbolToken),
  Literal(LiteralToken),
  EndOfFile {
    position: Position,
  },
  Invalid {
    position: Position,
  },
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Position {
  pub line: u32,
  pub column: u32,
}

impl Position {
  pub fn new(line: u32, column: u32) -> Self {
    Self { line, column }
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.line, self.column)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolToken {
  pub position: Position,
  pub symbol: Symbol,
}

impl Display for SymbolToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.symbol)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeywordToken {
  pub position: Position,
  pub keyword: Keyword,
}

impl Display for KeywordToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.keyword)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralToken {
  pub position: Position,
  pub literal: Literal,
}

impl Display for LiteralToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.literal)
  }
}

impl Token {
  pub fn position(&self) -> &Position {
    match self {
      Token::Symbol(SymbolToken { position, .. }) => position,
      Token::Keyword(KeywordToken { position, .. }) => position,
      Token::Literal(LiteralToken { position, .. }) => position,
      Token::EndOfFile { position, .. } => position,
      Token::Invalid { position, .. } => position,
    }
  }

  pub fn lexeme(&self) -> String {
    match self {
      Token::Symbol(SymbolToken { symbol, .. }) => symbol.lexeme(),
      Token::Keyword(KeywordToken { keyword, .. }) => keyword.lexeme(),
      Token::Literal(LiteralToken { literal, .. }) => literal.lexeme(),
      Token::EndOfFile { .. } => "[EOF]".into(),
      Token::Invalid { .. } => "[INV]".into(),
    }
  }

  pub fn reserved_word(value: &str, position: Position) -> Option<Self> {
    match value {
      // Keywords
      Keyword::IF => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::If,
      })),
      Keyword::ELSE => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Else,
      })),
      Keyword::FOR => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::For,
      })),
      Keyword::WHILE => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::While,
      })),
      Keyword::LOOP => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Loop,
      })),
      Keyword::RETURN => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Return,
      })),
      Keyword::SELF => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::_Self,
      })),
      Keyword::SUPER => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Super,
      })),
      Keyword::IMPORT => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Import,
      })),
      Keyword::EXPORT => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Export,
      })),
      Keyword::PUBLIC => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Public,
      })),
      Keyword::TYPE => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Type,
      })),
      Keyword::IMPL => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Impl,
      })),
      Keyword::AS => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::As,
      })),
      Keyword::TRAIT => Some(Token::Keyword(KeywordToken {
        position,
        keyword: Keyword::Trait,
      })),
      _ => None,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.lexeme())
  }
}
