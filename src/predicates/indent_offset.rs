use crate::{predicates::prelude::*, settings::IndentRule};

pub struct IndentOffset;

impl Predicate for IndentOffset {
  fn name(&self) -> &'static str { "indent-offset!" }

  fn parse<'a, 'tree>(
    &self,
    query: &Query,
    args: &'a [QueryPredicateArg],
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    _editor: &mut Editor,
  ) -> Result<(), Error> {
    ensure!(args.len() != 2, Error::nargs("2", args.len()));

    let offset_node_id = match &args[1] {
      QueryPredicateArg::Capture(cap_ix) => {
        let mut nodes = nodes_provider.nodes_for_cap_ix(*cap_ix);
        let node = nodes
          .next()
          .ok_or_else(|| Error::cap(query, *cap_ix, "no node captured"))?;
        if nodes.next().is_some() {
          bail!(Error::cap(
            query,
            *cap_ix,
            "multiple nodes captured for offset",
          ));
        }
        node.id()
      }
      QueryPredicateArg::String(s) => {
        bail!(Error::arg(1, "capture", format!("\"{s}\"")))
      }
    };

    let cap_ix = match &args[0] {
      QueryPredicateArg::Capture(cap_ix) => *cap_ix,
      QueryPredicateArg::String(s) => {
        bail!(Error::arg(0, "capture", format!("\"{s}\"")))
      }
    };

    let rule = IndentRule::Offset(offset_node_id);

    nodes_provider.nodes_for_cap_ix(cap_ix).into_iter().for_each(|node| {
      if let Some(old_rule) =
        settings.node_entry(node).or_default().set_indent_rule(rule)
      {
        log::warn!(
          "\"indent-rule\" overwritten for {}; old rule was \"{old_rule}\"",
          Displayer(node),
        );
      }
      log::trace!("set \"indent-rule\" to \"{rule}\" for {}", Displayer(node));
    });

    Ok(())
  }
}
