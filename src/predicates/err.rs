use {std::string::ToString, tree_sitter::Query};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("invalid argument #{ix}; expected {expected}, got {got}")]
  Arg { ix: usize, expected: String, got: String },

  #[error("error in capture \"{name}\": {msg}")]
  Cap { name: String, msg: String },

  #[error("invalid number of arguments; expected {expected}, got {got}")]
  Nargs { expected: String, got: String },

  #[error("invalid predicate operator \"{0}\"")]
  Op(String),
}

impl Error {
  pub fn arg(ix: usize, expected: impl ToString, got: impl ToString) -> Self {
    Self::Arg { ix, expected: expected.to_string(), got: got.to_string() }
  }

  pub fn cap(query: &Query, ix: u32, msg: impl ToString) -> Self {
    let name = query.capture_names()[ix as usize].clone();
    Self::Cap { name, msg: msg.to_string() }
  }

  pub fn nargs(expected: impl ToString, got: impl ToString) -> Self {
    Self::Nargs { expected: expected.to_string(), got: got.to_string() }
  }

  pub fn op(s: impl ToString) -> Self { Self::Op(s.to_string()) }
}
