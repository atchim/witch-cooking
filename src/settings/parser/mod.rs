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

trait Parser {
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
}

impl Default for Parsers {
  fn default() -> Self {
    let mut inner = ParsersInner::default();

    macro_rules! insert_settings {
      ($($setting:path),+ $(,)?) => {
        $({
          let setting = &$setting;
          inner.insert(setting.setting(), setting);
        })+
      };
    }

    insert_settings!(
      cpl::CplParser,
      ignored::Ignored,
      indent_rule::IndentRuleParser,
      indent_style::IndentStyle,
    );

    Self(inner)
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
