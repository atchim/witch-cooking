use {crate::predicates::prelude::*, ropey::RopeSlice, tree_sitter::Point};

/// Returns the number of leading [`ascii_whitespaces`] at `row` from `slice`.
///
/// [`ascii_whitespaces`]: char::is_ascii_whitespace
fn ascii_whitespaces(row: usize, slice: RopeSlice<'_>) -> usize {
  slice
    .line(row)
    .chars()
    .map_while(|ch| ch.is_ascii_whitespace().then_some(()))
    .count()
}

/// Attempts to return a [`Node`] that is located prior to `node`.
fn prev(mut node: Node<'_>) -> Option<Node<'_>> {
  loop {
    if let Some(prev) = node.prev_sibling() {
      break Some(prev);
    } else if let Some(parent) = node.parent() {
      node = parent;
    } else {
      break None;
    }
  }
}

pub struct Indent;

impl Predicate for Indent {
  fn name(&self) -> &'static str { "indent!" }

  fn parse<'a, 'tree>(
    &self,
    query: &Query,
    args: &'a [QueryPredicateArg],
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    settings: &mut Settings<'a, 'tree>,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let style = settings
      .indent_style()
      .ok_or_else(|| Error::other("\"indent-style\" not set"))?;

    for (ix, arg) in args.iter().enumerate() {
      let cap_ix = match arg {
        QueryPredicateArg::Capture(ix) => *ix,
        QueryPredicateArg::String(s) => {
          bail!(Error::arg(ix, "capture", format!("\"{s}\"")))
        }
      };

      for node in nodes_provider.nodes_for_cap_ix(cap_ix) {
        let rule = match settings
          .for_node(node)
          .and_then(|settings| settings.indent_rule())
        {
          None => {
            log::warn!(
              "no \"indent-rule\" setting for node {}",
              Displayer(node),
            );
            continue;
          }
          Some(rule) => rule,
        };

        let src = editor.src();

        use crate::settings::IndentRule::*;
        let indent = match rule {
          Absolute(n) => style.repeat(n as usize).to_string(),
          Offset(node_id) => {
            let mut node = *nodes_provider.node_for_id(node_id).unwrap();
            editor.sync(&mut node);
            let row = node.start_position().row;
            let whitespaces = ascii_whitespaces(row, src);
            let row_ch_ix = src.line_to_char(row);
            let offset = row_ch_ix + whitespaces;
            let indent = src.slice(row_ch_ix..offset);
            let start_ch = src.byte_to_char(node.start_byte());
            let align = " ".repeat(start_ch - offset);
            format!("{indent}{align}")
          }
          _ => {
            let mut parent = node.parent().ok_or_else(|| {
              Error::cap(
                query,
                cap_ix,
                format!("no parent node for {}", Displayer(node)),
              )
            })?;
            editor.sync(&mut parent);
            let row = parent.start_position().row;
            let whitespaces = ascii_whitespaces(row, src);
            let row_ch_ix = src.line_to_char(row);
            let indent = src.slice(row_ch_ix..row_ch_ix + whitespaces);
            match rule {
              Minus(n) => {
                let len = style.len() * n as usize;
                let ix =
                  indent.len_bytes().checked_sub(len).ok_or_else(|| {
                    Error::cap(
                      query,
                      cap_ix,
                      format!(
                        "unable to indent {} with rule \"{rule}\"",
                        Displayer(node),
                      ),
                    )
                  })?;
                format!("{}", indent.slice(..ix))
              }
              Plus(n) => format!("{indent}{}", style.repeat(n as usize)),
              _ => unreachable!(),
            }
          }
        };

        let mut node = *node;
        match prev(node) {
          None => {
            editor.sync(&mut node);
            let row = node.start_position().row;
            let whitespaces = ascii_whitespaces(row, src);
            let row_byte_ix = src.line_to_byte(row);
            editor.replace(
              &Range {
                start_byte: row_byte_ix,
                end_byte: row_byte_ix + whitespaces,
                start_point: Point { row, column: 0 },
                end_point: Point { row, column: whitespaces },
              },
              &indent,
            );
          }
          Some(mut prev) => {
            editor.sync(&mut node);
            editor.sync(&mut prev);
            let node_start_point = node.start_position();
            let prev_end_point = prev.end_position();
            let (start_byte, start_point, indent) = match node_start_point.row
              == prev_end_point.row
            {
              false => (
                src.line_to_byte(node_start_point.row),
                Point { row: node_start_point.row, column: 0 },
                indent,
              ),
              true => (prev.end_byte(), prev_end_point, format!("\n{indent}")),
            };
            editor.replace(
              &Range {
                start_byte,
                end_byte: node.start_byte(),
                start_point,
                end_point: node.start_position(),
              },
              &indent,
            );
          }
        }
      }
    }

    Ok(())
  }
}
