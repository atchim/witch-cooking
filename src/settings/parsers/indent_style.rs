use crate::settings::parsers::prelude::*;

pub struct IndentStyle;

impl Parser for IndentStyle {
  fn setting(&self) -> &'static str { "indent-style" }

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    scope: Scope,
    _nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error> {
    ensure!(query_prop.capture_id.is_none(), Error::CapExtra);

    let style = query_prop
      .value
      .as_ref()
      .ok_or_else(|| Error::value("indentation style", "no value"))?;

    if let Some(old_style) = settings.set_indent_style(style, scope) {
      log::warn!(
        "\"indent-style\" overwritten {scope}ly; old style was \"{}\"",
        old_style,
      );
    }

    log::trace!("{scope}ly set \"indent-style\" to \"{style}\"");

    Ok(())
  }
}
