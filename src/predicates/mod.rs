mod err;
mod indent_offset;
mod space;
mod spacer;

pub use err::Error;
use {
  crate::{
    editor::Editor,
    node_utils::Provider,
    settings::{Scope, Settings},
  },
  error_stack::Result,
  fnv::FnvHashMap,
  tree_sitter::{Query, QueryPredicate, QueryPredicateArg},
};

pub trait Predicate {
  fn name(&self) -> &'static str;

  fn parse<'a, 'tree>(
    &self,
    query: &Query,
    args: &'a [QueryPredicateArg],
    scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    editor: &mut Editor,
  ) -> Result<(), Error>;
}

type PredicatesInner = FnvHashMap<&'static str, &'static dyn Predicate>;

pub struct Predicates(PredicatesInner);

impl Predicates {
  pub fn parse<'a, 'tree>(
    &self,
    query: &Query,
    query_predicate: &'a QueryPredicate,
    scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let op = query_predicate.operator.as_ref();
    self.0.get(op).ok_or_else(|| Error::op(op))?.parse(
      query,
      &query_predicate.args,
      scope,
      nodes_provider,
      settings,
      editor,
    )
  }
}

impl Default for Predicates {
  fn default() -> Self {
    let mut inner = PredicatesInner::default();

    macro_rules! insert_predicates {
      ($($predicate:path),+ $(,)?) => {{
        $(
          let predicate = &$predicate;
          assert!(inner.insert(predicate.name(), predicate).is_none());
        )+
      }};
    }

    insert_predicates!(
      indent_offset::IndentOffset,
      space::Space,
      spacer::Spacer,
    );

    Predicates(inner)
  }
}

mod prelude {
  pub(super) use {
    super::{err::Error, Predicate},
    crate::{
      editor::Editor,
      node_utils::{Displayer, Provider, Walker},
      settings::{Scope, Settings},
    },
    error_stack::{bail, ensure, Result},
    tree_sitter::{Node, Query, QueryPredicateArg, Range},
  };
}
