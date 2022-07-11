use nom::{
    branch::alt, bytes::complete::tag, character::streaming::space1, sequence::preceded, IResult,
};

use crate::typess::string;

fn var(i: &str) -> IResult<&str, (&str, i16)> {
    let (p, _) = tag("var")(i)?;

    // variable name
    let (a, b) = string(p.trim())?;

    Ok((a, (b, 0)))
}

// test
fn obj(i: &str) -> IResult<&str, (&str, i16)> {
    let (p, _) = tag("object")(i)?;

    let (a, b) = string(p.trim())?;

    Ok((a, (b, 1)))
}

pub fn define(i: &str) -> IResult<&str, (&str, i16)> {
    let (d, _) = tag("define")(i)?;

    let (res, typ) = preceded(space1, alt((var, obj)))(d)?;

    Ok((res, typ))
}
