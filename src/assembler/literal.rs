use std::any::Any;
use crate::core::instruction::OpCode;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  // Literals
  OpCode { lexeme: OpCode },
  Identifier { lexeme: String },
  Number { lexeme: i64 },
}

impl Literal {
  pub const VOID: &'static str = "()";

  pub fn lexeme(&self) -> String {
    match self {
      Literal::OpCode { lexeme, .. } => lexeme.to_string(),
      Literal::Identifier { lexeme, .. } => lexeme.clone(),
      Literal::Number { lexeme, .. } => lexeme.to_string(),
    }
  }

  pub fn value(&self) -> Box<dyn Any> {
    match self.clone() {
      Literal::OpCode { lexeme } => Box::new(lexeme),
      Literal::Identifier { lexeme } => Box::new(lexeme),
      Literal::Number { lexeme } => Box::new(lexeme),
    }
  }
}
