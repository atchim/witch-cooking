use {
  crate::predicates::prelude::*,
  ropey::RopeSlice,
  std::{borrow::Cow, ops::RangeInclusive},
};

pub fn is_ascii_whitespace(s: &str) -> bool {
  for ch in s.chars() {
    if !ch.is_ascii_whitespace() {
      return false;
    }
  }
  !s.is_empty()
}

fn should_space(
  sep: &str,
  thresh: &RangeInclusive<usize>,
  s: RopeSlice<'_>,
) -> bool {
  if sep.is_empty() {
    return true;
  }

  let mut ch_ix = 0;
  let mut count = 0;
  let sep_ch_len = sep.chars().count();

  for ch in s.chars() {
    let sep_ch = unsafe { sep.chars().nth(ch_ix).unwrap_unchecked() };

    match ch == sep_ch {
      false => ch_ix = 0,
      true => ch_ix += 1,
    }

    if ch_ix == sep_ch_len {
      ch_ix = 0;
      count += 1;
    }
  }

  !thresh.contains(&count)
}

pub struct Space;

impl Predicate for Space {
  fn name(&self) -> &'static str { "space!" }

  fn parse<'a, 'tree>(
    &self,
    query: &Query,
    args: &'a [QueryPredicateArg],
    _scope: Scope,
    nodes_provider: &Provider<'_, 'tree>,
    _settings: &mut Settings<'a, 'tree>,
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

    let thresh = {
      let one_thresh = match args.peek() {
        Some(QueryPredicateArg::String(s)) => {
          args.next();
          arg_ix += 1;
          let thresh = s.parse::<usize>().map_err(|_| {
            Error::arg(arg_ix, "usize threshold", format!("\"{s}\""))
          })?;
          Some(thresh)
        }
        _ => None,
      };

      let upper_thresh = match args.peek() {
        Some(QueryPredicateArg::String(s)) => {
          args.next();
          arg_ix += 1;
          let thresh = s.parse::<usize>().map_err(|_| {
            Error::arg(arg_ix, "usize threshold", format!("\"{s}\""))
          })?;
          Some(thresh)
        }
        _ => None,
      };

      match (one_thresh, upper_thresh) {
        (None, None) => None,
        (None, Some(_)) => unreachable!(),
        (Some(x), None) => Some(1..=x),
        (Some(x), Some(y)) => Some(x..=y),
      }
    };

    if log::log_enabled!(log::Level::Warn)
      || log::log_enabled!(log::Level::Trace)
    {
      let thresh: Cow<'_, _> = thresh.as_ref().map_or("".into(), |thresh| {
        format!(" and threshold {thresh:?}").into()
      });
      match is_ascii_whitespace(sep) {
        false => {
          log::warn!("spacing with non-ASCII-whitespace \"{sep}\" {thresh}")
        }
        true => log::trace!("spacing with \"{sep}\"{thresh}"),
      }
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

      let s = editor.src().byte_slice(range.start_byte..range.end_byte);
      if thresh.as_ref().map_or(true, |thresh| should_space(sep, thresh, s)) {
        editor.replace(&range, sep);
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_space_() {
    macro_rules! should_space {
      ($sep:literal, $thresh:expr, $s:literal $(,)?) => {
        assert!(should_space($sep, &$thresh, RopeSlice::from($s)))
      };
    }

    macro_rules! should_not_space {
      ($sep:literal, $thresh:expr, $s:literal $(,)?) => {
        assert!(!should_space($sep, &$thresh, RopeSlice::from($s)))
      };
    }

    should_space!("", 1..=2, "");
    should_not_space!("-", 0..=0, "");
    should_space!("-", 1..=1, "");
    should_not_space!("-", 0..=1, "-");
    should_space!("-", 0..=1, "--");
    should_not_space!("-", 0..=2, "-_-");
    should_not_space!("--", 0..=2, "---_---");
  }
}
