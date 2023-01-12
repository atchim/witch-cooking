use crate::settings::parser::prelude::*;

pub struct Ignored;

impl Parser for Ignored {
  fn setting(&self) -> &'static str { "ignored" }

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error> {
    let cap_ix = query_prop.capture_id.ok_or(Error::CapMissing)?;

    if let Some(value) = query_prop.value.as_ref() {
      bail!(Error::value("no value", format!("\"{value}\"")));
    }

    nodes_provider.nodes_for_cap_ix(cap_ix.try_into().unwrap()).for_each(
      |node| {
        if settings.node_entry(node).or_default().ignore(true) {
          log::warn!("\"ignored\" overwritten for {}", NodeDisplayer(node));
        }
        log::trace!("ignoring {}", NodeDisplayer(node));
      },
    );

    Ok(())
  }
}
