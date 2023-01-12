use std::string::ToString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("capture not allowed")]
  CapExtra,

  #[error("missing capture")]
  CapMissing,

  #[error("invalid setting key \"{0}\"")]
  Key(String),

  #[error("node not allowed")]
  NodeExtra,

  #[error("missing node")]
  NodeMissing,

  #[error("{0}")]
  Other(String),

  #[error("invalid value; expected {expected}, got {got}")]
  Value { expected: String, got: String },
}

impl Error {
  pub fn key(s: impl ToString) -> Self { Self::Key(s.to_string()) }

  pub fn other(t: impl ToString) -> Self { Self::Other(t.to_string()) }

  pub fn value(expected: impl ToString, got: impl ToString) -> Self {
    Self::Value { expected: expected.to_string(), got: got.to_string() }
  }
}
