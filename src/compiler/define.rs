use nom::{
  IResult,
  bytes::complete::tag
};

pub fn define(i: &str) -> IResult<&str, &str> {
  tag("define ")(i)
}