mod err;
mod indent_offset;
mod predicate;
mod space;
mod space_all;

pub use self::{err::Error, predicate::Predicate};
use {
  super::{
    editor::Editor,
    node_utils::Provider as NodesProvider,
    settings::{MatchSettings, NodeToSettings},
  },
  fnv::FnvHashMap,
  tree_sitter::{Query, QueryPredicate},
};

type PredicatesInner = FnvHashMap<&'static str, &'static dyn Predicate>;

pub struct Predicates(PredicatesInner);

impl Predicates {
  pub fn apply<'tree>(
    &self,
    query: &Query,
    query_predicate: &QueryPredicate,
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    match_settings: &mut MatchSettings,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let op = query_predicate.operator.as_ref();
    self.0.get(op).ok_or_else(|| Error::op(op))?.apply(
      query,
      &query_predicate.args,
      nodes_provider,
      node_to_settings,
      match_settings,
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
      space_all::SpaceAll,
    );

    Predicates(inner)
  }
}
