use nom::{
  sequence::delimited,
  character::complete::char,
  bytes::complete::is_not,
  IResult
};

/// for strings (`""`) I guess?
pub fn string(i: &str) -> IResult<&str, &str> {
  delimited(char('"'), is_not("\""), char('"'))(i)
}