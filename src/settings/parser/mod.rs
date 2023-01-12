mod err;

pub use err::Error;
use {
  crate::{node_utils::Provider as NodesProvider, settings::Settings},
  error_stack::Result,
  tree_sitter::QueryProperty,
};

pub trait Parser {
  fn setting(&self) -> &'static str;

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    nodes_provider: &NodesProvider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error>;
}
