use crate::settings::{cpl::Cpl, parsers::prelude::*};

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

    if let Some(old_val) = settings.set_cpl(cpl, scope) {
      log::warn!("\"cpl\" overwritten {scope}ly; old cpl was \"{old_val}\"",);
    }

    log::trace!("{scope}ly set \"cpl\" to \"{cpl}\"");
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
      settings::{cpl::Error as CplErr, Parsers, Scope, Settings},
      Editor,
      Error as CrateErr,
    },
    error_stack::Result,
    ropey::{Rope, RopeSlice},
    std::num::IntErrorKind,
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
    setting_parsers.push(&CplParser);
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
  fn cpl_global() {
    let res = cook_debugging(
      "",
      "(#set! cpl 79) (#dbg!)",
      |_query, _args, scope, _provider, settings, _editor| {
        assert_eq!(scope, Scope::Global);
        assert!(
          matches!(settings.cpl(), Some(Cpl::Sane(value)) if value == 79)
        );
      },
    );
    assert!(res.is_ok());
  }

  #[test]
  fn cpl_local() {
    let res = cook_debugging(
      "fn foo() { bar(); }",
      "(#set! cpl 79) (function_item (#set! cpl 0) (#dbg!))",
      |_query, _args, scope, _provider, settings, _editor| {
        assert_eq!(scope, Scope::Local);
        assert!(matches!(settings.cpl(), Some(Cpl::Unlimited)));
      },
    );
    assert!(res.is_ok());
  }

  #[test]
  fn cpl_err_too_big() {
    let res = cook_debugging("", "(#set! cpl 133)", |_, _, _, _, _, _| {});
    match res {
      Err(err) => {
        let cpl_err = err.downcast_ref::<CplErr>().unwrap();
        assert!(matches!(cpl_err, CplErr::TooBig));
      }
      _ => unreachable!(),
    }
  }

  #[test]
  fn cpl_err_too_little() {
    let res = cook_debugging("", "(#set! cpl 29)", |_, _, _, _, _, _| {});
    match res {
      Err(err) => {
        let cpl_err = err.downcast_ref::<CplErr>().unwrap();
        assert!(matches!(cpl_err, CplErr::TooLittle));
      }
      _ => unreachable!(),
    }
  }

  #[test]
  fn cpl_err_parse_int() {
    let res = cook_debugging("", "(#set! cpl XXX)", |_, _, _, _, _, _| {});
    match res {
      Err(err) => {
        let cpl_err = err.downcast_ref::<CplErr>().unwrap();
        assert!(matches!(
          cpl_err,
          CplErr::ParseInt(err) if err.kind() == &IntErrorKind::InvalidDigit,
        ));
      }
      _ => unreachable!(),
    }
  }

  #[test]
  fn cpl_overwrite() {
    let res = cook_debugging(
      "fn foo() { bar(); }",
      "
        (#dbg! empty-cpl)
        (#set! cpl 79)
        (#dbg! with-cpl)
        (function_item
          (#set! cpl 0) ; It does not take effect.
          (#dbg! not-intuitive)
          (#set! cpl 90) ; It overwrites the last CPL setting.
          (#dbg! as-expected))
      ",
      |_query, args, scope, _provider, settings, _editor| {
        let n = match &args[0] {
          QueryPredicateArg::String(s) => s.as_ref(),
          _ => unreachable!(),
        };

        match n {
          "empty-cpl" => {
            assert_eq!(scope, Scope::Global);
            assert!(matches!(settings.cpl(), None));
          }
          "with-cpl" => {
            assert_eq!(scope, Scope::Global);
            assert!(
              matches!(settings.cpl(), Some(Cpl::Sane(value)) if value == 79),
            );
          }
          "not-intuitive" => {
            assert_eq!(scope, Scope::Local);
            assert!(
              matches!(settings.cpl(), Some(Cpl::Sane(value)) if value == 90),
            );
          }
          "as-expected" => {
            assert_eq!(scope, Scope::Local);
            assert!(
              matches!(settings.cpl(), Some(Cpl::Sane(value)) if value == 90),
            );
          }
          _ => unreachable!(),
        }
      },
    );
    assert!(res.is_ok());
  }
}