mod cpl;
mod err;
mod ignored;
mod indent_rule;
mod indent_style;

pub use err::Error;
use {
  crate::{
    node_utils::Provider,
    settings::{Scope, Settings},
  },
  error_stack::Result,
  fnv::FnvHashMap,
  tree_sitter::QueryProperty,
};

pub trait Parser {
  fn setting(&self) -> &'static str;

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error>;
}

type ParsersInner = FnvHashMap<&'static str, &'static dyn Parser>;

pub struct Parsers(ParsersInner);

impl Parsers {
  pub fn empty() -> Self { Self(Default::default()) }

  pub fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error> {
    let key = query_prop.key.as_ref();
    self.0.get(key).ok_or_else(|| Error::key(key))?.parse(
      query_prop,
      scope,
      nodes_provider,
      settings,
    )
  }

  pub fn push(
    &mut self,
    parser: &'static impl Parser,
  ) -> Option<&'static dyn Parser> {
    self.0.insert(parser.setting(), parser)
  }
}

impl Default for Parsers {
  fn default() -> Self {
    let mut parsers = Self::empty();

    macro_rules! insert {
      ($($parser:path),+ $(,)?) => {{
        $(assert!(parsers.push(&$parser).is_none());)+
      }};
    }

    insert!(
      cpl::CplParser,
      ignored::Ignored,
      indent_rule::IndentRuleParser,
      indent_style::IndentStyle,
    );

    parsers
  }
}

mod prelude {
  pub(super) use {
    super::{err::Error, Parser},
    crate::{
      node_utils::{Displayer as NodeDisplayer, Provider},
      settings::{Scope, Settings},
    },
    error_stack::{bail, ensure, IntoReport, Result, ResultExt},
    tree_sitter::QueryProperty,
  };
}
