pub(crate) use {
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

macro_rules! lines {
  () => { "\n" };
  ($($line:literal),* $(,)?) => { concat!($($line, "\n"),*) };
}

pub(crate) use lines;

pub fn cook_debugging<D, P, S>(
  src: &str,
  query_src: &str,
  debugger_fn: D,
  setting_parsers_fn: S,
  predicates_fn: P,
) -> Result<Rope, CrateErr>
where
  D: Fn(
    &Query,
    &[QueryPredicateArg],
    Scope,
    &Provider<'_, '_>,
    &Settings<'_, '_>,
    &Editor,
  ),
  P: FnOnce(&mut Predicates<'_>),
  S: FnOnce(&mut Parsers<'_>),
{
  let mut ts_parser = TsParser::new();
  let mut query_cursor = QueryCursor::new();
  let mut setting_parsers = Parsers::empty();
  setting_parsers_fn(&mut setting_parsers);
  let mut predicates = Predicates::empty();
  let debugger = Debugger::new("dbg!", debugger_fn);
  predicates.push(&debugger);
  predicates_fn(&mut predicates);
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

pub mod prelude {
  pub use super::*;
}
