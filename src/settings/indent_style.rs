use super::setting::prelude::*;

pub struct IndentStyle;

impl Setting for IndentStyle {
  fn name(&self) -> &'static str { "indent-style" }

  fn apply<'tree>(
    &self,
    query_prop: &QueryProperty,
    _nodes_provider: &NodesProvider<'_, 'tree>,
    _node_to_settings: &mut NodeToSettings<'tree>,
    match_settings: &mut MatchSettings,
  ) -> Result<(), Error> {
    if query_prop.capture_id.is_some() {
      bail!(Error::CapExtra);
    }

    let style = query_prop
      .value
      .as_ref()
      .ok_or_else(|| Error::value("indentation style", "no value"))?;

    if let Some(old_style) = match_settings.set_indent_style(style) {
      log::warn!(
        "\"indent-style\" overwritten; old style was \"{old_style}\"",
      );
    }

    log::trace!("set \"indent-style\" to \"{style}\"");

    Ok(())
  }
}
