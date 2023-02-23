use crate::predicates::prelude::*;

pub struct Debugger<F>
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
  name: &'static str,
  f: F,
}

impl<F> Debugger<F>
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
  pub fn new(name: &'static str, f: F) -> Self { Self { name, f } }
}

impl<F> Predicate for Debugger<F>
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
  fn name(&self) -> &'static str { self.name }

  fn parse<'a, 'tree>(
    &self,
    query: &Query,
    args: &'a [QueryPredicateArg],
    scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    (self.f)(query, args, scope, nodes_provider, settings, editor);
    Ok(())
  }
}
