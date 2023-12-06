use nom::{IResult, combinator::map_res, character::complete::digit1, Parser};



pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |out: &str| i64::from_str_radix(out, 10)).parse(input)
}