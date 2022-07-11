use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited, IResult};

/// for strings (`""`) I guess?
pub fn string(i: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(i)
}
