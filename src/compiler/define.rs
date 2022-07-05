use nom::{
  IResult,
  bytes::complete::tag,
  branch::alt,
  character::streaming::space1,
  sequence::preceded
};

use crate::typess::string;

fn var(i: &str) -> IResult<&str, &str> {
  let (p, _) = tag("var")(i)?;

  // variable name
  string(p.trim())
}

// test
fn obj(i: &str) -> IResult<&str, &str> {
  let (p, _) = tag("object")(i)?;

  string(p.trim())
}

pub fn define(i: &str) -> IResult<&str, &str> {
  let (d, _) = tag("define")(i)?;

  let (res, typ) = preceded(space1, alt((var, obj)))(d)?;

  Ok((res, typ))
}