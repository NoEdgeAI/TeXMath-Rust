use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, escaped},
    character::complete::{char, digit1, multispace0, none_of, one_of},
    combinator::{map, map_res, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    error::{ErrorKind, ParseError},
    IResult,
};

use super::node;

#[test]
fn test_parse_exp_list() {
    let test_case = "[ENumber \"1\", ENumber \"2\"]";
    assert_eq!(parse_exp_list(test_case), Ok(("", node::ExpList{list: vec![node::Exp::ENumber("1".to_string()), node::Exp::ENumber("2".to_string())]})));
}

fn parse_exp_list(input: &str) -> IResult<&str, node::ExpList> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('[')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp_list) = many0(parse_exp)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, node::ExpList{list: exp_list}))
}

#[test]
fn test_parse_exp() {
    let test_case = "EIdentifier \"x\"";
    assert_eq!(parse_exp(test_case), Ok(("", node::Exp::EIdentifier("x".to_string()))));

    let test_case = "ENumber \"123\"";
    assert_eq!(parse_exp(test_case), Ok(("", node::Exp::ENumber("123".to_string()))));

    let test_case = "EMathOperator \"sin\"";
    assert_eq!(parse_exp(test_case), Ok(("", node::Exp::EMathOperator("sin".to_string()))));
}

fn parse_exp(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;

    let (input, exp) = alt((
        parse_symbol,
        parse_number,
        parse_identifier,
        parse_math_operator,
        parse_delimited,
    ))(input)?;
    Ok((input, exp))
}

fn parse_tex_symbol_type(input: &str) -> IResult<&str, node::TeXSymbolType>{
    // Ord, Op, Bin, Rel, Open, Close, Pun, Accent, Fence, TOver, TUnder, Alpha, BotAccent, Rad
    alt((
        map(tag("Ord"), |_| node::TeXSymbolType::Ord),
        map(tag("Op"), |_| node::TeXSymbolType::Op),
        map(tag("Bin"), |_| node::TeXSymbolType::Bin),
        map(tag("Rel"), |_| node::TeXSymbolType::Rel),
        map(tag("Open"), |_| node::TeXSymbolType::Open),
        map(tag("Close"), |_| node::TeXSymbolType::Close),
        map(tag("Pun"), |_| node::TeXSymbolType::Pun),
        map(tag("Accent"), |_| node::TeXSymbolType::Accent),
        map(tag("Fence"), |_| node::TeXSymbolType::Fence),
        map(tag("TOver"), |_| node::TeXSymbolType::TOver),
        map(tag("TUnder"), |_| node::TeXSymbolType::TUnder),
        map(tag("Alpha"), |_| node::TeXSymbolType::Alpha),
        map(tag("BotAccent"), |_| node::TeXSymbolType::BotAccent),
        map(tag("Rad"), |_| node::TeXSymbolType::Rad),
    ))(input)
}

#[test]
fn test_parse_quoted_string() {
    // 正常情况，没有转义字符
    assert_eq!(
        parse_quoted_string("\"This is a test.\""),
        Ok(("", "This is a test.".to_string()))
    );

    // 包含转义引号
    assert_eq!(
        parse_quoted_string(r#" "This is a \"test\".""#),
        Ok(("", r#"This is a "test"."#.to_string()))
    );

    // 包含转义反斜杠
    assert_eq!(
        parse_quoted_string(r#" "This is a \\\\ test.""#),
        Ok(("", r#"This is a \\ test."#.to_string()))
    );
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('"')(input)?;
    
    // parse escaped string
    let (input, string) = 
    escaped_transform(none_of(r#"\""#), '\\', alt((
        map(char('"'), |_| '"'),
        map(char('\\'), |_| '\\'),
    )))(input)?;

    let (input, _) = char('"')(input)?;
    Ok((input, string))
}

#[test]
fn test_parse_symbol() {
    let test = r#"ESymbol Op "=""#;
    assert_eq!(parse_symbol(test), Ok(("", node::Exp::ESymbol(node::TeXSymbolType::Op, "=".to_string()))));
}
// symbol: ESymbol TeXSymbolType String
fn parse_symbol(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("ESymbol")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, symbol_type) = parse_tex_symbol_type(input)?;
    let (input, _) = multispace0(input)?;
    let (input, text_type) = parse_quoted_string(input)?;
    Ok((input, node::Exp::ESymbol(symbol_type, text_type)))
}


#[test]
fn test_parse_number() {
    let test_case = "ENumber \"123\"";
    assert_eq!(parse_number(test_case), Ok(("", node::Exp::ENumber("123".to_string()))));
}
// number: ENumber String
fn parse_number(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("ENumber")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, number) = parse_quoted_string(input)?;
    Ok((input, node::Exp::ENumber(number)))
}


#[test]
fn test_parse_identifier() {
    let test_case = "EIdentifier \"x\"";
    assert_eq!(parse_identifier(test_case), Ok(("", node::Exp::EIdentifier("x".to_string()))));
}

// identifier: EIdentifier String
fn parse_identifier(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("EIdentifier")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, identifier) = parse_quoted_string(input)?;
    Ok((input, node::Exp::EIdentifier(identifier)))
}

#[test]
fn test_parse_math_operator() {
    let test_case = "EMathOperator \"sin\"";
    assert_eq!(parse_math_operator(test_case), Ok(("", node::Exp::EMathOperator("sin".to_string()))));
}

// math_operator: EMathOperator String
fn parse_math_operator(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("EMathOperator")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, math_operator) = parse_quoted_string(input)?;
    Ok((input, node::Exp::EMathOperator(math_operator)))
}

#[test]
fn test_parse_delimited() {
    let test_case = "EDelimited \"(\" \")\" [ENumber \"1\", ENumber \"2\"]";
    assert_eq!(
        parse_delimited(test_case), 
        Ok(
            ("", 
            node::Exp::EDelimited("(".to_string(), ")".to_string(), 
            node::ExpList{list: vec![node::Exp::ENumber("1".to_string()), 
            node::Exp::ENumber("2".to_string())]}))
        ));
}

// delimited: EDelimited String String [Exp]
fn parse_delimited(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("EDelimited")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, left_delimiter) = parse_quoted_string(input)?;
    let (input, _) = multispace0(input)?;
    let (input, right_delimiter) = parse_quoted_string(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp_list) = parse_exp_list(input)?;
    Ok((input, node::Exp::EDelimited(left_delimiter, right_delimiter, exp_list)))
}

#[test]
fn test_parse_grouped() {
    let test_case = "EGrouped [ENumber \"1\", ENumber \"2\"]";
    assert_eq!(
        parse_grouped(test_case), 
        Ok(
            ("", 
            node::Exp::EGrouped(
            node::ExpList{list: vec![node::Exp::ENumber("1".to_string()), 
            node::Exp::ENumber("2".to_string())]}))
        ));
}

// grouped: EGrouped [Exp]
fn parse_grouped(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("EGrouped")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp_list) = parse_exp_list(input)?;
    Ok((input, node::Exp::EGrouped(exp_list)))
}

