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
  use {
    super::*,
    crate::{
      cook,
      node_utils::Provider,
      predicates::{Debugger, Predicates},
      settings::{Parsers, Scope, Settings},
      Editor,
      Error as CrateErr,
    },
    error_stack::Result,
    ropey::{Rope, RopeSlice},
    tree_sitter::{Parser as TsParser, Query, QueryCursor, QueryPredicateArg},
    tree_sitter_rust::language as rs_lang,
  };

  fn cook_debugging<'a, F>(
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
    let mut ts_parser = TsParser::new();
    let mut query_cursor = QueryCursor::new();
    let mut setting_parsers = Parsers::empty();
    setting_parsers.push(&Ignored);
    let mut predicates = Predicates::empty();
    let debugger = Debugger::new("dbg!", debugger_fn);
    predicates.push(&debugger);
    cook(
      &mut ts_parser,
      RopeSlice::from(src),
      rs_lang(),
      query_src,
      &mut query_cursor,
      &setting_parsers,
      &predicates,
    )
  }

  #[test]
  fn ignored() {
    let res = cook_debugging(
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
}
