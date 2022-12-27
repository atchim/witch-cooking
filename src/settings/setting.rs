use {
  super::{
    super::node_utils::Provider as NodesProvider, err::Error, MatchSettings,
    NodeToSettings,
  },
  tree_sitter::QueryProperty,
};

pub trait Setting {
  fn name(&self) -> &'static str;

  fn apply<'tree>(
    &self,
    query_prop: &QueryProperty,
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    match_settings: &mut MatchSettings,
  ) -> Result<(), Error>;
}

pub(super) mod prelude {
  pub(in super::super) use super::super::super::err::bail;
  pub use {
    super::{
      super::{
        super::node_utils::{
          Displayer as NodeDisplayer, Provider as NodesProvider,
        },
        err::Error,
        MatchSettings, NodeToSettings,
      },
      Setting,
    },
    tree_sitter::QueryProperty,
  };
}
