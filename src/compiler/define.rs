use nom::{
  IResult,
  bytes::complete::tag,
  branch::alt
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

  let def = d.trim();

  let (res, typ) = alt((var, obj))(def)?;

  Ok((res, typ))
}