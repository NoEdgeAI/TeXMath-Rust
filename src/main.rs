use nom::{
    IResult,
    sequence::tuple,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::map_res,
    multi::separated_list0,
};
use std::str::FromStr;

// 定义一个简单的解析器，解析形如 "123abc" 的字符串
fn parse_simple(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((take_while_m_n(1, 3, |c: char| c.is_digit(10)), tag("abc")))(input)
}

fn main() {
    let input = "123abc";
    match parse_simple(input) {
        Ok((remaining, result)) => println!("Parsed: {:?}, Remaining: {}", result, remaining),
        Err(e) => println!("Error parsing input: {:?}", e),
    }
}