use {
  crate::{
    node_utils::Provider as NodesProvider,
    settings::{err::Error, MatchSettings, NodeToSettings},
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

pub mod prelude {
  pub(in crate::settings) use crate::err::bail;
  pub use {
    crate::{
      node_utils::{Displayer as NodeDisplayer, Provider as NodesProvider},
      settings::{
        err::Error,
        setting::Setting,
        MatchSettings,
        NodeToSettings,
      },
    },
    tree_sitter::QueryProperty,
  };
}
