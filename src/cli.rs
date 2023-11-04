use {
  crate::err::Error,
  clap::{Parser, ValueEnum},
  detect_lang::Language as DlLang,
  std::path::PathBuf,
  tree_sitter::Language as TsLang,
};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Lang {
  #[cfg(feature = "bash")]
  Bash,
  #[cfg(feature = "c")]
  C,
  #[cfg(feature = "cpp")]
  Cpp,
  #[cfg(feature = "go")]
  Go,
  #[cfg(feature = "html")]
  Html,
  #[cfg(feature = "java")]
  Java,
  #[cfg(feature = "javascript")]
  JavaScript,
  #[cfg(feature = "python")]
  Python,
  #[cfg(feature = "rust")]
  Rust,
  #[cfg(feature = "toml")]
  Toml,
}

impl<'a> TryFrom<DlLang<'a>> for Lang {
  type Error = Error;

  fn try_from(lang: DlLang<'_>) -> Result<Self, Self::Error> {
    Ok(match lang.name() {
      #[cfg(feature = "bash")]
      "Shell" => Lang::Bash,
      #[cfg(feature = "c")]
      "C" => Lang::C,
      #[cfg(feature = "cpp")]
      "C++" => Lang::Cpp,
      #[cfg(feature = "go")]
      "Go" => Lang::Go,
      #[cfg(feature = "html")]
      "HTML" => Lang::Html,
      #[cfg(feature = "java")]
      "Java" => Lang::Java,
      #[cfg(feature = "javascript")]
      "JavaScript" => Lang::JavaScript,
      #[cfg(feature = "markdown")]
      "Markdown" => Lang::Markdown,
      #[cfg(feature = "python")]
      "Python" => Lang::Python,
      #[cfg(feature = "rust")]
      "Rust" => Lang::Rust,
      #[cfg(feature = "toml")]
      "TOML" => Lang::Toml,
      name => return Err(Error::lang_unsupported(name)),
    })
  }
}

impl From<Lang> for TsLang {
  fn from(lang: Lang) -> Self {
    match lang {
      #[cfg(feature = "bash")]
      Lang::Bash => tree_sitter_bash::language(),
      #[cfg(feature = "c")]
      Lang::C => tree_sitter_c::language(),
      #[cfg(feature = "cpp")]
      Lang::Cpp => tree_sitter_cpp::language(),
      #[cfg(feature = "go")]
      Lang::Go => tree_sitter_go::language(),
      #[cfg(feature = "html")]
      Lang::Html => tree_sitter_html::language(),
      #[cfg(feature = "java")]
      Lang::Java => tree_sitter_java::language(),
      #[cfg(feature = "javascript")]
      Lang::JavaScript => tree_sitter_javascript::language(),
      #[cfg(feature = "python")]
      Lang::Python => tree_sitter_python::language(),
      #[cfg(feature = "rust")]
      Lang::Rust => tree_sitter_rust::language(),
      #[cfg(feature = "toml")]
      Lang::Toml => tree_sitter_toml::language(),
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
