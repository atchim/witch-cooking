use crate::settings::parsers::prelude::*;

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

#[cfg(test)]
mod tests {
  use {super::*, crate::query_testing::prelude::*};

  #[inline]
  fn cook_debugging_ignored<F>(
    src: &str,
    query_src: &str,
    debugger_fn: F,
  ) -> Result<Rope, CrateErr>
  where
    F: Fn(
      &Query,
      &[QueryPredicateArg],
      Scope,
      &Provider<'_, '_>,
      &Settings<'_, '_>,
      &Editor,
    ),
  {
    cook_debugging(
      src,
      query_src,
      debugger_fn,
      |setting_parsers| {
        setting_parsers.push(&Ignored);
      },
      |_| {},
    )
  }

  #[test]
  fn ignored() {
    let res = cook_debugging_ignored(
      "fn foo() {}",
      "
        ((identifier) @id (#set! @id ignored))
        ( (function_item name: _ @name (#dbg! fn-name @name)) @fn
          (#dbg! fn @fn))
      ",
      |_query, args, _scope, provider, settings, _editor| {
        let marker = match &args[0] {
          QueryPredicateArg::String(s) => s.as_ref(),
          _ => unreachable!(),
        };

        let cap_ix = match &args[1] {
          QueryPredicateArg::Capture(ix) => *ix,
          _ => unreachable!(),
        };

        let mut cap_nodes = provider.nodes_for_cap_ix(cap_ix);
        let cap_node = cap_nodes.next().unwrap();
        assert!(cap_nodes.next().is_none());

        match marker {
          "fn-name" => assert!(settings.for_node(cap_node).unwrap().ignored()),
          "fn" => assert!(settings.for_node(cap_node).is_none()),
          _ => unreachable!(),
        }
      },
    );
    assert!(res.is_ok());
  }

  #[test]
  fn ignored_err_cap_missing() {
    let res =
      cook_debugging_ignored("", "(#set! ignored)", |_, _, _, _, _, _| {});
    match res {
      Err(err) => {
        let parse_err = err.downcast_ref::<Error>().unwrap();
        assert!(matches!(parse_err, Error::CapMissing));
      }
      _ => unreachable!(),
    }
  }

  #[test]
  fn ignored_err_value() {
    let res = cook_debugging_ignored(
      "fn foo() {}",
      "((function_item) @fn (#set! @fn ignored false))",
      |_, _, _, _, _, _| {},
    );
    match res {
      Err(err) => {
        let parse_err = err.downcast_ref::<Error>().unwrap();
        assert!(matches!(parse_err, Error::Value { .. }));
      }
      _ => unreachable!(),
    }
  }
}
