use crate::assembler::lexer::Lexer;
use crate::assembler::parser::Parser;

mod util;
mod lexer;
mod parser;
mod token;
mod keyword;
mod literal;
mod symbol;

pub struct Interpreter {
  lexer: Lexer,
  parser: Parser,
}

impl Default for Interpreter {
  fn default() -> Self {
    Self::new()
  }
}

impl Interpreter {
  pub fn new() -> Interpreter {
    let error_handler = Handle::new(ErrorHandler::new());
    let lexer = Lexer::new(error_handler.clone());
    let parser = Parser::new(error_handler.clone());
    let binder = Binder::new(error_handler.clone());

    Self {
      error_handler,
      lexer,
      parser,
      binder,
      show_tokens: false,
      show_tree: false,
    }
  }

  pub fn run(&mut self, source: String) -> Result<String, KonError> {
    self.error_handler.get_mut().clear();
    let tokens = self.lexer.lex(&source);

    if self.show_tokens {
      println!("{tokens:#?}");
      self.show_tokens = false;
    }

    let tree = self.parser.parse(&tokens);

    if self.show_tree {
      print!("{}", tree); // tree has trailing newline due to recursive impl
      self.show_tree = false;
    }

    let bound_tree = self.binder.bind(tree.root);

    self.error_handler.get().try_report_errors()?;

    // print!("Result: ");

    match bound_tree.evaluate() {
      Ok(result) => {
        if let Some(&value) = result.downcast_ref::<i64>() {
          return Ok(value.to_string());
        }

        if let Some(value) = result.downcast_ref::<String>() {
          return Ok(value.clone());
        }

        Err(KonError::Evaluation("resultant data type not supported".into()))
      }
      Err(error) => Err(KonError::Evaluation(format!("{error}"))),
    }
  }

  pub fn show_tree(&mut self) {
    self.show_tree = true;
  }

  pub fn show_next_tokens(&mut self) {
    self.show_tokens = true;
  }
}
