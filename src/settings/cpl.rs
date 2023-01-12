use std::{
  fmt,
  num::{NonZeroU8, ParseIntError},
  str::FromStr,
};

#[derive(Clone, Copy, Default)]
pub struct Cpl(Option<NonZeroU8>);

impl Cpl {
  #[inline]
  pub fn value(&self) -> Option<NonZeroU8> { self.0 }
}

impl From<u8> for Cpl {
  #[inline]
  fn from(n: u8) -> Self {
    let inner = match n {
      0 => None,
      _ => Some(unsafe { NonZeroU8::new_unchecked(n) }),
    };
    Cpl(inner)
  }
}

impl fmt::Display for Cpl {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let n: u8 = match self.value() {
      None => 0,
      Some(n) => n.into(),
    };
    write!(f, "{n}")
  }
}

impl FromStr for Cpl {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let n = s.parse::<u8>()?;
    Ok(n.into())
  }
}
