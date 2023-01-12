use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
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
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let op = chars.next().ok_or(Error::Empty)?;
    match chars.next() {
      None => Err(Error::Value),
      Some(ch) if ch.is_ascii_digit() => Ok(()),
      Some(ch) => Err(Error::NonDigit(ch)),
    }?;
    let n = s[1..].parse()?;
    use IndentRule::*;
    Ok(match op {
      '+' => Plus(n),
      '-' => Minus(n),
      '=' => Absolute(n),
      ch => return Err(Error::Op(ch)),
    })
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
        test!($op, "-1", Err(Error::NonDigit('-')));
        test!($op, "0", Ok(IndentRule::$enum_var(0)));
        test!($op, "", Err(Error::Value));
        test!(
          $op,
          "256",
          Err(Error::U8(err))
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
    test!("", Err(Error::Empty));
    test!("#", Err(Error::Value));
    test!("#0", Err(Error::Op('#')));
  }
}
