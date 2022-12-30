use super::predicate::prelude::*;

pub fn is_ascii_whitespace(s: &str) -> bool {
  for ch in s.chars() {
    if !ch.is_ascii_whitespace() {
      return false;
    }
  }
  !s.is_empty()
}

pub struct Space;

impl Predicate for Space {
  fn name(&self) -> &'static str { "space!" }

  fn apply<'tree>(
    &self,
    query: &Query,
    args: &[QueryPredicateArg],
    nodes_provider: &NodesProvider<'_, 'tree>,
    _node_to_settings: &mut NodeToSettings<'tree>,
    _match_settings: &mut MatchSettings,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let mut arg_ix = 0;
    let mut args = args.iter().peekable();

    let sep = match args.peek() {
      Some(QueryPredicateArg::String(s)) => {
        args.next();
        arg_ix += 1;
        s.as_ref()
      }
      _ => " ",
    };

    match is_ascii_whitespace(sep) {
      false => log::warn!("spacing with non-ASCII-whitespace \"{sep}\""),
      true => log::trace!("spacing with \"{sep}\""),
    }

    let a_cap_ix = match args.next() {
      None => bail!(Error::arg(arg_ix, "capture", "none")),
      Some(QueryPredicateArg::Capture(ix)) => *ix,
      Some(QueryPredicateArg::String(s)) => {
        bail!(Error::arg(arg_ix, "capture", format!("\"{s}\"")))
      }
    };

    let b_cap_ix = match args.next() {
      None => bail!(Error::arg(arg_ix, "capture", "none")),
      Some(QueryPredicateArg::Capture(ix)) => *ix,
      Some(QueryPredicateArg::String(s)) => {
        bail!(Error::arg(arg_ix, "capture", format!("\"{s}\"")));
      }
    };

    let mut a_nodes = nodes_provider.nodes_for_cap_ix(a_cap_ix);
    let mut b_nodes = nodes_provider.nodes_for_cap_ix(b_cap_ix);

    loop {
      let (mut a_node, mut b_node) = match (a_nodes.next(), b_nodes.next()) {
        (None, None) => break,
        (None, Some(_)) => {
          log::warn!(
            "\"{}\" did not capture, but \"{}\" did",
            query.capture_names()[a_cap_ix as usize],
            query.capture_names()[b_cap_ix as usize],
          );
          break;
        }
        (Some(_), None) => {
          log::warn!(
            "\"{}\" did capture, but \"{}\" did not",
            query.capture_names()[a_cap_ix as usize],
            query.capture_names()[b_cap_ix as usize],
          );
          break;
        }
        (Some(prev), Some(cur)) => (*prev, *cur),
      };

      editor.sync(&mut a_node);
      editor.sync(&mut b_node);

      let range = Range {
        start_byte: a_node.end_byte(),
        end_byte: b_node.start_byte(),
        start_point: a_node.end_position(),
        end_point: b_node.start_position(),
      };

      editor.replace(&range, sep);
    }

    Ok(())
  }
}
