use crate::settings::{cpl::Cpl, parser::prelude::*};

pub struct CplParser;

impl Parser for CplParser {
  fn setting(&self) -> &'static str { "cpl" }

  fn parse<'a, 'tree>(
    &self,
    query_prop: &'a QueryProperty,
    scope: Scope,
    _nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
  ) -> Result<(), Error> {
    ensure!(query_prop.capture_id.is_none(), Error::CapExtra);

    let s = query_prop
      .value
      .as_ref()
      .ok_or_else(|| Error::value("cpl", "no value"))?;

    let cpl = s
      .parse::<Cpl>()
      .report()
      .change_context_lazy(|| Error::value("cpl", format!("\"{s}\"")))?;

    if let Some(old_val) = settings.set_cpl(cpl, scope).value() {
      log::warn!("\"cpl\" overwritten {scope}ly; old cpl was \"{old_val}\"",);
    }

    log::trace!("{scope}ly set \"cpl\" to \"{cpl}\"");
    Ok(())
  }
}
