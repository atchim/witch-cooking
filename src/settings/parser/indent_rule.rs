use crate::settings::{indent_rule::IndentRule, parser::prelude::*};

pub struct IndentRuleParser;

impl Parser for IndentRuleParser {
  fn setting(&self) -> &'static str { "indent-rule" }

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error> {
    let cap_ix = query_prop.capture_id.ok_or(Error::CapMissing)?;

    let s = query_prop
      .value
      .as_ref()
      .ok_or_else(|| Error::value("indentation rule", "no value"))?;

    let rule = s.parse::<IndentRule>().map_err(Error::other)?;

    nodes_provider.nodes_for_cap_ix(cap_ix.try_into().unwrap()).for_each(
      |node| {
        if let Some(old_rule) =
          settings.node_entry(node).or_default().set_indent_rule(rule)
        {
          log::warn!(
            "\"indent-rule\" overwritten for {}; old rule was \"{old_rule}\"",
            NodeDisplayer(node),
          );
        }
        log::trace!(
          "set \"indent-rule\" to \"{rule}\" for {}",
          NodeDisplayer(node),
        );
      },
    );

    Ok(())
  }
}
