use enumflags2::BitFlags;
use super::{
  keyword::Keyword,
  literal::Literal,
  symbol::Symbol,
  token::{KeywordToken, LiteralToken, Position, SymbolToken, Token, TokenDiscriminants},
  util::token_provider::{Next, TokenProvider},
};

#[derive(Debug, PartialEq, Eq)]
enum DelimiterType {
  Paren,
  Curly,
  Angled,
  Square,
}

#[derive(Debug, Eq)]
struct Delimiter {
  position: Position,
  delimiter: DelimiterType,
}

impl Delimiter {
  pub fn lexeme(&self) -> String {
    match self.delimiter {
      DelimiterType::Paren => "(".into(),
      DelimiterType::Curly => "{".into(),
      DelimiterType::Angled => "<".into(),
      DelimiterType::Square => "[".into(),
    }
  }

  fn position(&self) -> &Position {
    &self.position
  }
}

impl PartialEq for Delimiter {
  fn eq(&self, other: &Self) -> bool {
    self.delimiter == other.delimiter
  }
}

impl TryFrom<Token> for Delimiter {
  type Error = ();

  fn try_from(token: Token) -> Result<Self, Self::Error> {
    let Token::Symbol(SymbolToken { symbol, .. }) = token else {
      return Err(());
    };

    if !matches!(
      symbol,
      Symbol::RightParenthesis | Symbol::RightAngledBracket | Symbol::RightCurlyBracket | Symbol::RightSquareBracket
    ) {
      return Err(());
    }

    let delimiter = match symbol {
      Symbol::LeftParenthesis => DelimiterType::Paren,
      Symbol::LeftAngledBracket => DelimiterType::Angled,
      Symbol::LeftCurlyBracket => DelimiterType::Curly,
      Symbol::LeftSquareBracket => DelimiterType::Square,
      Symbol::RightParenthesis => DelimiterType::Paren,
      Symbol::RightAngledBracket => DelimiterType::Angled,
      Symbol::RightCurlyBracket => DelimiterType::Curly,
      Symbol::RightSquareBracket => DelimiterType::Square,
      _ => return Err(()),
    };

    Ok(Self {
      delimiter,
      position: token.position().clone(),
    })
  }
}

pub struct Parser {
  delimiter_stack: Vec<Delimiter>,
}

impl Parser {
  pub fn new(error_handler: Handle<ErrorHandler>) -> Self {
    Self {
      delimiter_stack: Default::default(),
    }
  }

  pub fn parse(&mut self, tokens: &[Token]) -> SyntaxTree {
    let mut tokens = TokenProvider::new(tokens);
    let root = self.expression(&mut tokens);

    let next = tokens.peek().cloned();
    match next {
      Next::Token(t) => {
        self.error_handler.get_mut().push(InterpreterError::ParseError {
          position: t.position().clone(),
          message: format!("Expected EOF but got `{}`", t),
        });
        SyntaxTree {
          root,
          eof: Token::EndOfFile {
            position: Position::new(t.position().line, t.position().column + 1),
          },
        }
      }
      Next::EndOfFile { position } => SyntaxTree {
        root,
        eof: Token::EndOfFile { position },
      },
      Next::EndOfStream { position } => SyntaxTree {
        root,
        eof: Token::EndOfFile { position },
      },
    }
  }

  // TODO: fix angled bracket false positive for 1 > 3
  fn is_rogue_delimiter(&mut self, token: &Next<&Token>) -> bool {
    let Next::Token(token) = token.cloned() else {
      return false;
    };

    let Ok(delim) = token.try_into() else {
      return false;
    };

    !self.delimiter_stack.ends_with(&[delim])
  }

  fn match_token_types(&mut self, tokens: &mut TokenProvider, types: BitFlags<TokenDiscriminants>) -> Option<Token> {
    let mut peeked = tokens.peek();
    // check for rogue delimiters
    if self.is_rogue_delimiter(&peeked) {
      let Next::Token(Token::Symbol(SymbolToken { position, symbol })) = &peeked else {
        return None;
      };

      self.error(InterpreterError::UnmatchedDelimiter {
        position: position.clone(),
        delimiter: symbol.lexeme(),
      });

      tokens.next(); // consume the delimiter
      peeked = tokens.peek();
    }

    // check if no tokens
    let Next::Token(peeked) = peeked else {
      return None;
    };

    // check if matches params
    let discrm = TokenDiscriminants::from(peeked);
    if types.contains(discrm) {
      Some(peeked.clone())
    } else {
      None
    }
  }

  fn match_symbols(&mut self, tokens: &mut TokenProvider, types: BitFlags<Symbol>) -> Option<SymbolToken> {
    if let Some(Token::Symbol(symbol_token)) = self.match_token_types(tokens, TokenDiscriminants::Symbol.into()) {
      if types.contains(symbol_token.symbol) {
        if let Next::Token(Token::Symbol(symbol_token)) = tokens.next() {
          return Some(symbol_token.clone());
        }
      }
    }

    None
  }

  fn expression(&mut self, tokens: &mut TokenProvider) -> Expression {
    self.or(tokens)
  }

  fn or(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.and(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::DoublePipe.into()) {
      let right_operand = Box::new(self.and(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn and(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.comparison(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::DoubleAmpersand.into()) {
      let right_operand = Box::new(self.comparison(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn comparison(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.term(tokens);

    while let Some(operator) = self.match_symbols(
      tokens,
      Symbol::DoubleEquals
        | Symbol::ExclamationPointEquals
        | Symbol::LeftAngledBracket
        | Symbol::RightAngledBracket
        | Symbol::LeftAngledBracketEquals
        | Symbol::RightAngledBracketEquals,
    ) {
      let right_operand = Box::new(self.term(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn term(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.factor(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::Plus | Symbol::Minus) {
      let right_operand = Box::new(self.factor(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn factor(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.unary(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::Asterisk | Symbol::ForwardSlash) {
      let right_operand = Box::new(self.unary(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  // fn power(&mut self, tokens: &mut TokenProvider) -> Expression {
  //   let mut expression = self.unary(tokens);

  //   while let Some(operator) = self.match_symbols(tokens, Symbol::Caret.into())
  // {     let right_operand = Box::new(self.unary(tokens));
  //     expression = Expression::Binary {
  //       left_operand: Box::new(expression),
  //       operator,
  //       right_operand,
  //     };
  //   }

  //   expression
  // }

  fn unary(&mut self, tokens: &mut TokenProvider) -> Expression {
    if let Some(operator) = self.match_symbols(tokens, Symbol::ExclamationPoint | Symbol::Minus) {
      let operand = Box::new(self.unary(tokens));
      return Expression::Unary { operator, operand };
    }

    self.primary(tokens)
  }

  fn primary(&mut self, tokens: &mut TokenProvider) -> Expression {
    let next_token = tokens.next().cloned();
    match next_token {
      Next::Token(token) => {
        match &token {
          Token::Literal(token) => return Expression::Literal { token: token.clone() },
          Token::Symbol(SymbolToken { position, symbol }) => match symbol {
            Symbol::LeftParenthesis => {
              self.delimiter_stack.push(Delimiter {
                delimiter: DelimiterType::Paren,
                position: position.clone(),
              });
              let operand = Box::new(self.expression(tokens));
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightParenthesis);
              return Expression::Grouping { operand };
            }
            Symbol::LeftCurlyBracket => {
              self.delimiter_stack.push(Delimiter {
                delimiter: DelimiterType::Curly,
                position: position.clone(),
              });
              let operand = Box::new(self.expression(tokens));
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightCurlyBracket);
              self.delimiter_stack.pop();
              return Expression::Grouping { operand };
            }
            _ => {}
          },
          _ => {}
        }

        self.error(InterpreterError::ParseError {
          position: token.position().clone(),
          message: format!("Expected expression but got `{}`", tokens.previous_valid()),
        });

        Expression::Literal {
          token: LiteralToken {
            position: token.position().clone(),
            literal: Literal::Void,
          },
        }
      }
      Next::EndOfFile { position } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          position: Position::new(prev.position().line, prev.position().column + prev.lexeme().len() as u32),
          message: format!("Expected expression after `{}`", prev),
        });

        Expression::Literal {
          token: LiteralToken {
            position,
            literal: Literal::Void,
          },
        }
      }
      Next::EndOfStream { position } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          position: Position::new(prev.position().line, prev.position().column + prev.lexeme().len() as u32),
          message: format!("Expected expression after `{}`", prev),
        });

        Expression::Literal {
          token: LiteralToken {
            position,
            literal: Literal::Void,
          },
        }
      }
    }
  }

  fn pair_delimiter(&mut self, tokens: &mut TokenProvider, delimiter: Symbol) -> Token {
    let unmatched = self.delimiter_stack.pop().expect("expected Some(delimiter)");
    match self.check_delimiter(tokens, &delimiter) {
      Ok(matches) => match tokens.next().cloned() {
        Next::Token(token) => {
          if matches {
            token
          } else {
            let delimiter = Token::Symbol(SymbolToken {
              position: token.position().clone(),
              symbol: delimiter,
            });
            self.error(InterpreterError::UnmatchedDelimiter {
              position: unmatched.position().clone(),
              delimiter: unmatched.lexeme(),
            });
            delimiter
          }
        }
        Next::EndOfFile { position } | Next::EndOfStream { position } => {
          let delimiter = Token::Symbol(SymbolToken {
            position,
            symbol: delimiter,
          });
          self.error(InterpreterError::UnmatchedDelimiter {
            position: unmatched.position().clone(),
            delimiter: unmatched.lexeme(),
          });
          delimiter
        }
      },
      Err(position) => {
        let delimiter = Token::Symbol(SymbolToken {
          position,
          symbol: delimiter,
        });
        self.error(InterpreterError::UnmatchedDelimiter {
          position: unmatched.position().clone(),
          delimiter: unmatched.lexeme(),
        });
        delimiter
      }
    }
  }

  fn check_delimiter(&mut self, tokens: &mut TokenProvider, delimiter: &Symbol) -> Result<bool, Position> {
    match tokens.peek() {
      Next::Token(Token::Symbol(symbol_token)) => Ok(symbol_token.symbol == *delimiter),
      Next::Token(token) => Err(token.position().to_owned()),
      Next::EndOfFile { position } | Next::EndOfStream { position } => Err(position),
    }
  }

  fn error(&mut self, error: InterpreterError) {
    self.error_handler.get_mut().push(error.clone());
  }

  #[allow(unused)]
  fn synchronize(&mut self, tokens: &mut TokenProvider) {
    let mut token = tokens.next();
    while let Next::Token(next_token) = token {
      if matches!(
        next_token,
        Token::Symbol(SymbolToken {
          symbol: Symbol::Semicolon,
          ..
        })
      ) {
        return;
      }

      if matches!(
        next_token,
        Token::Keyword(KeywordToken {
          keyword: Keyword::If
            | Keyword::For
            | Keyword::While
            | Keyword::Loop
            | Keyword::Return
            | Keyword::Type
            | Keyword::Impl,
          ..
        })
      ) {
        return;
      }

      token = tokens.next();
    }
  }
}
