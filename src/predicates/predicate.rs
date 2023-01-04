use {
  crate::{
    editor::Editor,
    node_utils::Provider as NodesProvider,
    predicates::err::Error,
    settings::{MatchSettings, NodeToSettings},
  },
  tree_sitter::{Query, QueryPredicateArg},
};

pub trait Predicate {
  fn name(&self) -> &'static str;

  fn apply<'tree>(
    &self,
    query: &Query,
    args: &[QueryPredicateArg],
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    match_settings: &mut MatchSettings,
    editor: &mut Editor,
  ) -> Result<(), Error>;
}

pub mod prelude {
  pub(in crate::predicates) use crate::err::{bail, ensure};
  #[allow(unused_imports)]
  pub use {
    crate::{
      editor::Editor,
      node_utils::{
        Displayer as NodeDisplayer,
        Jumper,
        Provider as NodesProvider,
        Walker,
      },
      predicates::{err::Error, predicate::Predicate},
      settings::{MatchSettings, NodeToSettings},
    },
    tree_sitter::{Node, Point, Query, QueryPredicateArg, Range},
  };
}
