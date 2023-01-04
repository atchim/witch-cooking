use {
  crate::settings::setting::prelude::*,
  std::{fmt, num::ParseIntError, str::FromStr},
};

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum IndentRuleErr {
  #[error("empty indent rule")]
  Empty,

  #[error("invalid non-digit character \"{0}\" at index 1")]
  NonDigit(char),

  #[error("invalid operator \"{0}\"")]
  Op(char),

  #[error("{0}")]
  U8(#[from] ParseIntError),

  #[error("missing value for operator")]
  Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndentRule {
  Absolute(u8),
  Minus(u8),
  Offset(usize),
  Plus(u8),
}

impl fmt::Display for IndentRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use IndentRule::*;
    write!(f, "{}", match self {
      Absolute(n) => format!("={n}"),
      Minus(n) => format!("-{n}"),
      Offset(node_id) => format!("#{node_id}"),
      Plus(n) => format!("+{n}"),
    })
  }
}

impl FromStr for IndentRule {
  type Err = IndentRuleErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let op = chars.next().ok_or(IndentRuleErr::Empty)?;
    match chars.next() {
      None => Err(IndentRuleErr::Value),
      Some(ch) if ch.is_ascii_digit() => Ok(()),
      Some(ch) => Err(IndentRuleErr::NonDigit(ch)),
    }?;
    let n = s[1..].parse()?;
    use IndentRule::*;
    Ok(match op {
      '+' => Plus(n),
      '-' => Minus(n),
      '=' => Absolute(n),
      ch => return Err(IndentRuleErr::Op(ch)),
    })
  }
}

pub struct IndentRuleSetting;

impl Setting for IndentRuleSetting {
  fn name(&self) -> &'static str { "indent-rule" }

  fn apply<'tree>(
    &self,
    query_prop: &QueryProperty,
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    _match_settings: &mut MatchSettings,
  ) -> Result<(), Error> {
    let cap_ix = query_prop.capture_id.ok_or(Error::CapMissing)?;

    let s = query_prop
      .value
      .as_ref()
      .ok_or_else(|| Error::value("indentation rule", "no value"))?;

    let rule = s.parse::<IndentRule>().map_err(Error::other)?;

    nodes_provider.nodes_for_cap_ix(cap_ix.try_into().unwrap()).for_each(
      |node| {
        if let Some(old_rule) =
          node_to_settings.entry(node).or_default().set_indent_rule(rule)
        {
          log::warn!(
            "\"indent-rule\" overwritten for {}; old rule was \"{old_rule}\"",
            NodeDisplayer(node),
          );
        }
        log::trace!(
          "set \"indent-rule\" to \"{rule}\" for {}",
          NodeDisplayer(node),
        );
      },
    );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, std::num::IntErrorKind};

  #[test]
  fn from_str() {
    macro_rules! test {
      ($s:expr, $res:pat $(=> $clause:expr)? $(,)?) => {
        assert!(matches!($s.parse::<IndentRule>(), $res $(if $clause)?))
      };

      ($op:literal, $s:literal, $res:pat $(=> $clause:expr)? $(,)?) => {
        test!(concat!($op, $s), $res $(=> $clause)?)
      };

      ($op:literal => $enum_var:ident) => {{
        test!($op, "-1", Err(IndentRuleErr::NonDigit('-')));
        test!($op, "0", Ok(IndentRule::$enum_var(0)));
        test!($op, "", Err(IndentRuleErr::Value));
        test!(
          $op,
          "256",
          Err(IndentRuleErr::U8(err))
          => err.kind() == &IntErrorKind::PosOverflow,
        );
      }};

      ($op:literal) => {
        match $op {
          "+" => test!("+" => Plus),
          "-" => test!("-" => Minus),
          "=" => test!("=" => Absolute),
          op => panic!("invalid operator: {op}"),
        }
      };
    }

    test!("=");
    test!("-");
    test!("+");
    test!("", Err(IndentRuleErr::Empty));
    test!("#", Err(IndentRuleErr::Value));
    test!("#0", Err(IndentRuleErr::Op('#')));
  }
}
