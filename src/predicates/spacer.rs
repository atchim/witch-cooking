use {
  crate::predicates::{prelude::*, space::is_ascii_whitespace},
  std::borrow::Cow,
};

fn spacer<'tree>(
  node: Node<'tree>,
  sep: &str,
  settings: &Settings<'_, 'tree>,
  editor: &mut Editor,
) {
  let mut cursor = node.walk();
  let mut walker = Walker::from(&mut cursor).filter(|item| {
    let node = item.node();
    node.child_count() == 0
      && node
        .parent()
        .and_then(|parent| {
          settings.for_node(&parent).map(|settings| !settings.ignored())
        })
        .unwrap_or(true)
  });

  let mut prev = match walker.next() {
    None => return,
    Some(item) => item.into(),
  };
  editor.sync(&mut prev);

  for item in walker {
    let mut node = item.into();
    editor.sync(&mut node);
    let range = Range {
      start_byte: prev.end_byte(),
      end_byte: node.start_byte(),
      start_point: prev.end_position(),
      end_point: node.start_position(),
    };
    editor.replace(&range, sep);
    editor.sync_last(&mut node);
    prev = node;
  }
}

pub struct Spacer;

impl Predicate for Spacer {
  fn name(&self) -> &'static str { "spacer!" }

  fn parse<'a, 'tree>(
    &self,
    _query: &Query,
    args: &'a [QueryPredicateArg],
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let (cap_ix, sep) = match args.len() {
      n @ 1 | n @ 2 => {
        let cap_ix = match &args[0] {
          QueryPredicateArg::Capture(capture) => *capture,
          QueryPredicateArg::String(s) => {
            bail!(Error::arg(0, "capture", format!("\"{s}\"")));
          }
        };

        let sep = match n {
          1 => " ".into(),
          _ => match &args[1] {
            QueryPredicateArg::Capture(_) => {
              bail!(Error::arg(1, "string", "capture"));
            }
            QueryPredicateArg::String(s) => Cow::Owned(s.to_string()),
          },
        };

        (cap_ix, sep)
      }
      n => bail!(Error::nargs("1, 2 or 3", n)),
    };

    match is_ascii_whitespace(sep.as_ref()) {
      false => log::warn!("spacing with non-ASCII-whitespace \"{sep}\""),
      true => log::trace!("spacing with \"{sep}\""),
    }

    nodes_provider.nodes_for_cap_ix(cap_ix).for_each(|node| {
      spacer(*node, &sep, settings, editor);
    });

    Ok(())
  }
}
