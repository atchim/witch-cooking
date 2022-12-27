use super::predicate::prelude::*;

fn arg_to_single_node<'a, 'tree>(
  arg: (usize, &QueryPredicateArg),
  nodes_provider: &'a NodesProvider<'_, 'tree>,
) -> Result<&'a Node<'tree>, Error> {
  match arg {
    (arg_ix, QueryPredicateArg::Capture(cap_ix)) => {
      let mut nodes = nodes_provider.nodes_for_cap_ix(*cap_ix);

      let node = nodes.next().ok_or_else(|| {
        Error::arg(
          arg_ix,
          "capture capturing single node",
          "capture capturing no node",
        )
      })?;

      ensure!(
        nodes.next().is_none(),
        Error::arg(
          arg_ix,
          "capture capturing one node only",
          "capture capturing multiple nodes",
        ),
      );

      Ok(node)
    }
    (arg_ix, QueryPredicateArg::String(s)) => {
      Err(Error::arg(arg_ix, "capture", format!("\"{s}\"")))
    }
  }
}

pub struct Space;

impl Predicate for Space {
  fn name(&self) -> &'static str {
    "space!"
  }

  fn apply<'tree>(
    &self,
    _query: &Query,
    args: &[QueryPredicateArg],
    nodes_provider: &NodesProvider<'_, 'tree>,
    _node_to_settings: &mut NodeToSettings<'tree>,
    _match_settings: &mut MatchSettings,
    editor: &mut Editor,
  ) -> Result<(), Error> {
    let mut args = args.iter().enumerate().peekable();

    let sep = match args.peek() {
      Some((_, QueryPredicateArg::String(s))) => {
        args.next();
        s.as_ref()
      }
      _ => " ",
    };

    log::trace!("spacing with \"{sep}\"");

    let args_len = args.len();
    ensure!(
      args_len % 2 == 0,
      Error::nargs("even number of capture pairs", args_len),
    );

    loop {
      let mut prev = match args.next() {
        None => break,
        Some(arg) => *arg_to_single_node(arg, nodes_provider)?,
      };
      editor.sync(&mut prev);

      let mut next =
        *arg_to_single_node(args.next().unwrap(), nodes_provider)?;
      editor.sync(&mut next);

      let range = Range {
        start_byte: prev.end_byte(),
        end_byte: next.start_byte(),
        start_point: prev.end_position(),
        end_point: next.start_position(),
      };
      editor.replace(&range, sep);
    }

    Ok(())
  }
}
