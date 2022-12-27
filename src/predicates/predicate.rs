use {
  super::{
    super::{
      editor::Editor,
      node_utils::Provider as NodesProvider,
      settings::{MatchSettings, NodeToSettings},
    },
    err::Error,
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

pub(super) mod prelude {
  pub(in super::super) use super::super::super::err::{bail, ensure};
  pub use {
    super::{
      super::{
        super::{
          editor::Editor,
          node_utils::{
            Displayer as NodeDisplayer,
            Jumper,
            Provider as NodesProvider,
            Walker,
          },
          settings::{MatchSettings, NodeToSettings},
        },
        err::Error,
      },
      Predicate,
    },
    tree_sitter::{Node, Point, Query, QueryPredicateArg, Range},
  };
}
