use {
  crate::err::Error,
  clap::{Parser, ValueEnum},
  detect_lang::Language as DlLang,
  std::path::PathBuf,
  tree_sitter::Language as TsLang,
};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Lang {
  #[cfg(feature = "rust")]
  Rust,
}

impl<'a> TryFrom<DlLang<'a>> for Lang {
  type Error = Error;

  fn try_from(lang: DlLang<'_>) -> Result<Self, Self::Error> {
    Ok(match lang.name() {
      #[cfg(feature = "rust")]
      "Rust" => Lang::Rust,
      name => return Err(Error::lang_unsupported(name)),
    })
  }
}

impl From<Lang> for TsLang {
  fn from(lang: Lang) -> Self {
    match lang {
      #[cfg(feature = "rust")]
      Lang::Rust => tree_sitter_rust::language(),
    }
  }
}

/// Cooking the source code.
#[derive(Debug, Parser)]
pub struct Opts {
  /// Language to parse.
  #[arg(short, value_enum)]
  pub lang: Option<Lang>,

  /// Query file.
  #[arg(short)]
  pub query: PathBuf,

  /// Source file.
  pub src: Option<PathBuf>,
}
