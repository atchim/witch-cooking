use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("{0}")]
  ParseInt(#[from] ParseIntError),

  #[error("value is too little")]
  TooLittle,

  #[error("value is too big")]
  TooBig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaneValue(u8);

impl SaneValue {
  const MAX: u8 = 128;
  const MIN: u8 = 32;
}

impl From<SaneValue> for u8 {
  #[inline]
  fn from(value: SaneValue) -> Self { value.0 }
}

impl PartialEq<u8> for SaneValue {
  fn eq(&self, other: &u8) -> bool { self.0 == *other }
}

impl TryFrom<u8> for SaneValue {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      _ if value < Self::MIN => Err(Error::TooLittle),
      _ if value > Self::MAX => Err(Error::TooBig),
      _ => Ok(Self(value)),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cpl {
  Unlimited,
  Sane(SaneValue),
}

impl From<Cpl> for u8 {
  #[inline]
  fn from(cpl: Cpl) -> Self {
    match cpl {
      Cpl::Unlimited => 0,
      Cpl::Sane(value) => value.into(),
    }
  }
}

impl TryFrom<u8> for Cpl {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Unlimited),
      _ => Ok(Self::Sane(SaneValue::try_from(value)?)),
    }
  }
}

impl fmt::Display for Cpl {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", u8::from(*self))
  }
}

impl FromStr for Cpl {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let n = s.parse::<u8>()?;
    let cpl = n.try_into()?;
    Ok(cpl)
  }
}
