use {
  std::fmt::Display,
  tree_sitter::{LanguageError, QueryError},
};

macro_rules! bail {
  ($err:expr) => {
    return Err($err)
  };
}

macro_rules! ensure {
  ($cond:expr, $err:expr $(,)?) => {
    if !$cond {
      return Err($err);
    }
  };
}

pub(super) use {bail, ensure};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("{0}")]
  Lang(LanguageError),

  #[error("language \"{0}\" is unsupported")]
  LangUnsupported(String),

  #[error("could not detect language")]
  LangUndetected,

  #[error("failed during execution")]
  Other,

  #[error("no input piped")]
  Pipe,

  #[error("failed to apply predicate \"{op}\" from pattern #{pat_ix}")]
  Predicate { op: String, pat_ix: usize },

  #[error("{0}")]
  Query(QueryError),

  #[error("could not open query file")]
  QueryFile,

  #[error("failed to apply setting \"{key}\" from pattern #{pat_ix}")]
  Setting { key: String, pat_ix: usize },

  #[error("could not open source file")]
  SrcFile,
}

impl Error {
  pub fn lang_unsupported(lang: impl Display) -> Self {
    Self::LangUnsupported(lang.to_string())
  }

  pub fn predicate(op: impl ToString, pat_ix: usize) -> Self {
    Error::Predicate { op: op.to_string(), pat_ix }
  }

  pub fn setting(pat_ix: usize, key: impl ToString) -> Self {
    Error::Setting { key: key.to_string(), pat_ix }
  }
}
