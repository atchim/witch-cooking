use super::setting::prelude::*;

pub struct Ignored;

impl Setting for Ignored {
  fn name(&self) -> &'static str { "ignored" }

  fn apply<'tree>(
    &self,
    query_prop: &QueryProperty,
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    _match_settings: &mut MatchSettings,
  ) -> Result<(), Error> {
    let cap_ix = query_prop.capture_id.ok_or(Error::CapMissing)?;

    if let Some(value) = query_prop.value.as_ref() {
      bail!(Error::value("no value", format!("\"{value}\"")));
    }

    nodes_provider.nodes_for_cap_ix(cap_ix.try_into().unwrap()).for_each(
      |node| {
        if node_to_settings.entry(node).or_default().ignore(true) {
          log::warn!("\"ignored\" overwritten for {}", NodeDisplayer(node));
        }
        log::trace!("ignoring {}", NodeDisplayer(node));
      },
    );

    Ok(())
  }
}
