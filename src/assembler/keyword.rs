#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
  Org,
}

impl Keyword {
  pub const AS: &'static str = "org";

  pub fn lexeme(&self) -> String {
    match self {
      Self::If => Self::IF.into(),
      Self::Else => Self::ELSE.into(),
      Self::For => Self::FOR.into(),
      Self::While => Self::WHILE.into(),
      Self::Loop => Self::LOOP.into(),
      Self::Return => Self::RETURN.into(),
      Self::_Self => Self::SELF.into(),
      Self::_SelfType => Self::SELF_TYPE.into(),
      Self::Super => Self::SUPER.into(),
      Self::Export => Self::EXPORT.into(),
      Self::Import => Self::IMPORT.into(),
      Self::Public => Self::PUBLIC.into(),
      Self::Type => Self::TYPE.into(),
      Self::Impl => Self::IMPL.into(),
      Self::As => Self::AS.into(),
      Self::Trait => Self::TRAIT.into(),
      Self::True => Self::TRUE.into(),
      Self::False => Self::FALSE.into(),
    }
  }
}
