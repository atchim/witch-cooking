mod jumper;
mod walker;

pub use {
  jumper::{Item as JumperItem, Jumper},
  walker::{Item as WalkerItem, Walker},
};

use {
  fnv::FnvHashMap,
  std::{borrow::Cow, fmt},
  tree_sitter::{Node, QueryMatches, TextProvider},
};

pub struct Displayer<'a, 'tree>(pub &'a Node<'tree>);

impl<'a, 'tree> fmt::Display for Displayer<'a, 'tree> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let kind = {
      let mut kind: Cow<'_, str> = self.0.kind().into();
      if !self.0.is_named() {
        kind = format!("\"{kind}\"").into();
      }
      kind
    };
    let start_point = self.0.start_position();
    let end_point = self.0.end_position();
    write!(
      f,
      "({kind} ({} {}) ({} {}))",
      start_point.row, start_point.column, end_point.row, end_point.column,
    )
  }
}

pub type CapIxToNodes<'tree> = FnvHashMap<u32, Vec<Node<'tree>>>;
pub type IdToNode<'tree> = FnvHashMap<usize, Node<'tree>>;

pub struct Provider<'a, 'tree> {
  cap_ix_to_nodes: &'a CapIxToNodes<'tree>,
  id_to_node: &'a IdToNode<'tree>,
}

impl<'a, 'tree> Provider<'a, 'tree> {
  #[inline]
  pub fn new(
    cap_ix_to_nodes: &'a CapIxToNodes<'tree>,
    ix_to_node: &'a IdToNode<'tree>,
  ) -> Self {
    Self { cap_ix_to_nodes, id_to_node: ix_to_node }
  }

  pub fn nodes_for_cap_ix(
    &self,
    ix: u32,
  ) -> impl Iterator<Item = &Node<'tree>> + '_ {
    self.cap_ix_to_nodes.get(&ix).map(Vec::as_slice).unwrap_or(&[]).iter()
  }

  #[inline]
  pub fn node_for_id(&self, id: usize) -> Option<&Node<'tree>> {
    self.id_to_node.get(&id)
  }

  pub fn nodes_for_match(&self) -> impl Iterator<Item = &Node<'tree>> + '_ {
    self.cap_ix_to_nodes.values().flatten()
  }
}

pub type PatIxToMatchNodes<'tree> =
  FnvHashMap<usize, Vec<CapIxToNodes<'tree>>>;

pub struct Matches<'tree> {
  pat_ix_to_match_nodes: PatIxToMatchNodes<'tree>,
  id_to_node: IdToNode<'tree>,
}

impl<'tree> Matches<'tree> {
  pub fn iter(
    &self,
  ) -> impl Iterator<Item = (usize, &[CapIxToNodes<'tree>])> + '_ {
    (0..self.pat_ix_to_match_nodes.len()).filter_map(|ix| {
      self
        .pat_ix_to_match_nodes
        .get(&ix)
        .map(|match_nodes| (ix, match_nodes.as_slice()))
    })
  }

  #[inline]
  pub fn id_to_node(&self) -> &IdToNode<'tree> {
    &self.id_to_node
  }
}

impl<'provider, 'tree, T> From<QueryMatches<'provider, 'tree, T>>
  for Matches<'tree>
where
  T: TextProvider<'provider>,
{
  fn from(query_matches: QueryMatches<'provider, 'tree, T>) -> Self {
    let mut pat_ix_to_match_nodes = PatIxToMatchNodes::default();
    let mut id_to_node = IdToNode::default();
    query_matches.for_each(|query_match| {
      let mut cap_ix_to_nodes = CapIxToNodes::default();
      query_match.captures.iter().for_each(|capture| {
        let node = capture.node;
        cap_ix_to_nodes.entry(capture.index).or_default().push(node);
        id_to_node.entry(node.id()).or_insert(node);
      });
      pat_ix_to_match_nodes
        .entry(query_match.pattern_index)
        .or_default()
        .push(cap_ix_to_nodes)
    });
    Self { pat_ix_to_match_nodes, id_to_node }
  }
}
