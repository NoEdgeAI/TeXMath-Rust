use super::node;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::{char, digit1, multispace0, none_of}, combinator::map, error::ErrorKind, Err, IResult
};

// ast reader [Exp ...]
pub fn read_ast(ast: &str) -> Result<Vec<node::Exp>, String> {
    match parse_exp_list(ast) {
        Ok((_, e)) => {
            Ok(e)
        },
        Err(e) => {
            let msg = format!("Parse error: {:?}", e);
            Result::Err(msg)
        }
    }
}



#[test]
fn test_parse_exp_list() {
    let test_case = "[ENumber \"1\", ENumber \"2\"]";
    let (output, exp) = parse_exp_list(test_case).unwrap();
    assert_eq!(
        (output.trim(), exp),
        ("",
        vec![
            node::Exp::ENumber("1".to_string()),
            node::Exp::ENumber("2".to_string())
        ]
        )
    );
}

fn parse_indelimited(input: &str) -> IResult<&str, Vec<node::InEDelimited>> {
    let mut input = input;
    (input, _) = multispace0(input)?;
    (input, _) = char('[')(input)?;
    let mut exp_list = Vec::<node::InEDelimited>::new();

    // 判断是否为空
    (input, _ ) = multispace0(input)?;
    if input.starts_with(']'){
        (input, _) = char(']')(input)?;
        return Ok((input, exp_list));
    }

    // 如果input不是mut的话, 每次循环会shadow input, 从而导致input的值不变
    // 陷入死循环
    loop{
        (input, _) = multispace0(input)?;

        if input.starts_with("Left"){
            let (tmp, left) = parse_left(input)?;
            input = tmp;
            exp_list.push(left);
        }else if input.starts_with("Right"){
            let (tmp, right) = parse_right(input)?;
            input = tmp;
            exp_list.push(right);
        }else{
            return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
        }

        (input, _) = multispace0(input)?;

        if input.starts_with(']'){
            break;
        }

        (input, _) = char(',')(input)?;
        (input, _) = multispace0(input)?;
    }

    (input, _) = char(']')(input)?;
    Ok((input, exp_list))
}

// [Exp, Exp, Exp ...]
fn parse_exp_list(input: &str) -> IResult<&str, Vec<node::Exp>> {
    let mut input = input;
    (input, _) = multispace0(input)?;
    (input, _) = char('[')(input)?;
    let mut exp_list = Vec::new();

    // 判断是否为空
    (input, _ ) = multispace0(input)?;
    if input.starts_with(']'){
        (input, _) = char(']')(input)?;
        return Ok((input, exp_list));
    }

    // 如果input不是mut的话, 每次循环会shadow input, 从而导致input的值不变
    // 陷入死循环
    loop{
        (input, _) = multispace0(input)?;

        let (tmp , exp) = parse_exp(input)?;
        input = tmp;
        exp_list.push(exp);
        (input, _) = multispace0(input)?;

        if input.starts_with(']'){
            break;
        }

        (input, _) = char(',')(input)?;
        (input, _) = multispace0(input)?;
    }

    (input, _) = char(']')(input)?;
    Ok((input, exp_list))
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
    // 往前多看几个字符, 可以避免回溯, 提高效率

    if input.starts_with("ESymbol"){
        // symbol
        let (input, exp) = parse_symbol(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ENumber"){
        // number
        let (input, exp) = parse_number(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EIdentifier"){
        // identifier
        let (input, exp) = parse_identifier(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EMathOperator"){
        // math_operator
        let (input, exp) = parse_math_operator(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EText"){
        // text
        let (input, exp) = parse_text(input)?;
        return Ok((input, exp));    
    }else if input.starts_with("EDelimited"){
        // delimited
        let (input, exp) = parse_delimited(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EGrouped"){
        // grouped
        let (input, exp) = parse_grouped(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ERoot"){
        // root
        let (input, exp) = parse_root(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ESqrt"){
        // sqrt
        let (input, exp) = parse_sqrt(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EFraction"){
        // fraction
        let (input, exp) = parse_fraction(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ESubsup"){
        // subsup
        let (input, exp) = parse_subsup(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ESub"){
        // sub
        let (input, exp) = parse_sub(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ESuper"){
        // super
        let (input, exp) = parse_super(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EOver"){
        // over
        let (input, exp) = parse_over(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EUnderover"){
        // under
        let (input, exp) = parse_under_over(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EUnder"){
        // under_over
        let (input, exp) = parse_under(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EPhantom"){
        // phantom
        let (input, exp) = parse_phantom(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EBoxed"){
        // boxed
        let (input, exp) = parse_boxed(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EScaled"){
        // scaled
        let (input, exp) = parse_scaled(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EStyled"){
        // styled
        let (input, exp) = parse_styled(input)?;
        return Ok((input, exp));
    }else if input.starts_with("EArray"){
        // array
        let (input, exp) = parse_array(input)?;
        return Ok((input, exp));
    }else if input.starts_with("ESpace"){
        // space
        let (input, exp) = parse_space(input)?;
        return Ok((input, exp));
    }
    
    
    return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
}


fn parse_tex_symbol_type(input: &str) -> IResult<&str, node::TeXSymbolType>{
    // Ord, Op, Bin, Rel, Open, Close, Pun, Accent, Fence, TOver, TUnder, Alpha, BotAccent, Rad
    alt((
        map(tag("Ord"), |_| node::TeXSymbolType::Ord),
        map(tag("Open"), |_| node::TeXSymbolType::Open),
        map(tag("Op"), |_| node::TeXSymbolType::Op),
        map(tag("Bin"), |_| node::TeXSymbolType::Bin),
        map(tag("Rel"), |_| node::TeXSymbolType::Rel),
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
        Ok(("", r#"This is a \"test\"."#.to_string()))
    );

    // 包含转义反斜杠
    assert_eq!(
        parse_quoted_string(r#" "This is a \\\\ test.""#),
        Ok(("", r#"This is a \\\\ test."#.to_string()))
    );

    assert_eq!(
        parse_quoted_string(r#""\8722""#),
        Ok(("", r#"\8722"#.to_string()))
    );

    assert_eq!(
        parse_quoted_string(r#""\"""#),
        Ok(("", r#"\""#.to_string()))
    );
    assert_eq!(
        parse_quoted_string(r#""\\""#),
        Ok(("", r#"\\"#.to_string()))
    );
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = char('"')(output)?;
    
    // empty
    if output.starts_with('"') {
        (output, _) = char('"')(output)?;
        return Ok((output, "".to_string()));
    }


    // 读取字符串, 直到遇到"且前面没有转义字符
    let mut res = String::new();
    loop {
        // 如果遇到转义字符
        if output.starts_with("\\") {
            res.push_str("\\");

            // 跳过转义字符
            output = &output[1..];

            let c = output.chars().next().unwrap();
            res.push(c);
            output = &output[1..];
            continue;
        }

        // 如果遇到引号 -> 结束
        if output.starts_with('"') {
            break;
        }

        // 读取一个字符
        let (tmp, c) = none_of("\"")(output)?;
        output = tmp;
        res.push(c);
    }

    (output, _) = char('"')(output)?;
    Ok((output, res))
}

#[test]
fn test_parse_symbol() {
    let test = r#"ESymbol Op "=""#;
    assert_eq!(parse_symbol(test), Ok(("", node::Exp::ESymbol(node::TeXSymbolType::Op, "=".to_string()))));
    let test = r#"ESymbol Op "\8722""#;
    assert_eq!(parse_symbol(test), Ok(("", node::Exp::ESymbol(node::TeXSymbolType::Op, "\\8722".to_string()))));
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

    let test_case = r#"EMathOperator """#;
    assert_eq!(parse_math_operator(test_case), Ok(("", node::Exp::EMathOperator("".to_string()))))
}

// math_operator: EMathOperator String
fn parse_math_operator(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EMathOperator")(output)?;
    (output, _) = multispace0(output)?;
    let (output, math_operator) = parse_quoted_string(output)?;
    Ok((output, node::Exp::EMathOperator(math_operator)))
}

#[test]
fn test_parse_text_type() {
    let test_case = "TextNormal";
    assert_eq!(parse_text_type(test_case), Ok(("", node::TextType::TextNormal)));
}

// text_type: TextType
fn parse_text_type(input: &str) -> IResult<&str, node::TextType> {
    alt((
        map(tag("TextNormal"), |_| node::TextType::TextNormal),

        map(tag("TextBoldItalic"), |_| node::TextType::TextBoldItalic),
        map(tag("TextBoldScript"), |_| node::TextType::TextBoldScript),
        map(tag("TextBoldFraktur"), |_| node::TextType::TextBoldFraktur),
        map(tag("TextBold"), |_| node::TextType::TextBold),

        map(tag("TextItalic"), |_| node::TextType::TextItalic),
        map(tag("TextMonospace"), |_| node::TextType::TextMonospace),

        map(tag("TextSansSerifItalic"), |_| node::TextType::TextSansSerifItalic),
        map(tag("TextSansSerifBoldItalic"), |_| node::TextType::TextSansSerifBoldItalic),
        map(tag("TextSansSerifBold"), |_| node::TextType::TextSansSerifBold),
        map(tag("TextSansSerif"), |_| node::TextType::TextSansSerif),

        map(tag("TextDoubleStruck"), |_| node::TextType::TextDoubleStruck),
        map(tag("TextScript"), |_| node::TextType::TextScript),
        map(tag("TextFraktur"), |_| node::TextType::TextFraktur),



    ))(input)
}

#[test]
fn test_parse_text() {
    let test_case = "EText TextNormal \"This is a test.\"";
    assert_eq!(
        parse_text(test_case), 
        Ok(
            ("", 
            node::Exp::EText(node::TextType::TextNormal, "This is a test.".to_string()))
        ));
    let test_case = r#"
    EText TextNormal "0"
    "#;

    println!("{:?}", parse_text(test_case));
}

// text: EText TextType String
fn parse_text(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = tag("EText")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, text_type) = parse_text_type(input)?;
    let (input, _) = multispace0(input)?;
    let (input, text) = parse_quoted_string(input)?;
    Ok((input, node::Exp::EText(text_type, text)))
}

#[test]
fn test_parse_delimited() {
    let test_case = r#"
    EDelimited
    "|"
    "|"
    [ Right (EFraction NormalFrac (EIdentifier "H") (EIdentifier "K"))
    ]
    "#;
    
    let (output, res) = parse_delimited(test_case).unwrap();
        
    assert_eq!(
        (output.trim(), res), 
        ("", 
        node::Exp::EDelimited("|".to_string(), "|".to_string(), 
        vec![node::InEDelimited::Right(
            node::Exp::EFraction(node::FractionType::NormalFrac,
            Box::new(node::Exp::EIdentifier("H".to_string())),
            Box::new(node::Exp::EIdentifier("K".to_string()))))]))    
    )
}

// delimited: EDelimited String String [Exp]
fn parse_delimited(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EDelimited")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, left_delimiter) = parse_quoted_string(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    let (tmp, right_delimiter) = parse_quoted_string(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    let (output, exp_list) = parse_indelimited(output)?;
    Ok((output, node::Exp::EDelimited(left_delimiter, right_delimiter, exp_list)))
}

#[test]
fn test_parse_grouped() {
    let test_case = "EGrouped [ENumber \"1\", ENumber \"2\"]";
    assert_eq!(
        parse_grouped(test_case), 
        Ok(
            ("", 
            node::Exp::EGrouped(
                vec![
                    node::Exp::ENumber("1".to_string()), 
                    node::Exp::ENumber("2".to_string())
                ]
            ))
        ));

    let test_case = r#"
    EGrouped  [ ESuper (EIdentifier "b") (ENumber "2")
              , ESymbol Bin "\8722"
              , ENumber "4"
              , EIdentifier "a"
              , EIdentifier "c"
              ]
    "#;
    println!("{:?}", parse_grouped(test_case));
}

// grouped: EGrouped [Exp]
fn parse_grouped(input: &str) -> IResult<&str, node::Exp> {
    let mut ouput = input;

    (ouput, _) = multispace0(ouput)?;
    (ouput, _) = tag("EGrouped")(ouput)?;
    (ouput, _) = multispace0(ouput)?;
    let (tmp, exp_list) = parse_exp_list(ouput)?;
    ouput = tmp;
    Ok((ouput, node::Exp::EGrouped(exp_list)))
}


#[test]
fn test_parse_root() {
    let test_case = "ERoot (ENumber \"1\") (ENumber \"2\")";
    assert_eq!(
        parse_root(test_case), 
        Ok(
            ("", 
            node::Exp::ERoot(Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string()))))
        ));
}

// ERoot (Exp) (Exp)
fn parse_root(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ERoot")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp1) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp2) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, node::Exp::ERoot(Box::new(exp1), Box::new(exp2))))
}

#[test]
fn test_parse_sqrt() {
    let test_case = "ESqrt (ENumber \"1\")";
    assert_eq!(
        parse_sqrt(test_case), 
        Ok(
            ("", 
            node::Exp::ESqrt(Box::new(node::Exp::ENumber("1".to_string()))))
        ));
}

#[test]
fn test_parse_fraction() {
    let test_case = "EFraction NormalFrac (ENumber \"1\") (ENumber \"2\")";
    assert_eq!(
        parse_fraction(test_case), 
        Ok(
            ("", 
            node::Exp::EFraction(node::FractionType::NormalFrac, 
            Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string()))))
        ));
    
    let test_case = r#"
    EFraction
    NormalFrac
    (EGrouped
       [ ESymbol Op "\8722"
       , EIdentifier "b"
       , ESymbol Bin "\177"
       , ESqrt
           (EGrouped
              [ ESuper (EIdentifier "b") (ENumber "2")
              , ESymbol Bin "\8722"
              , ENumber "4"
              , EIdentifier "a"
              , EIdentifier "c"
              ])
       ])
    (EGrouped [ ENumber "2" , EIdentifier "a" ])
    "#;

    println!("{:?}", parse_fraction(test_case));
}

fn parse_fraction_type(input: &str) -> IResult<&str, node::FractionType> {
    alt((
        map(tag("NormalFrac"), |_| node::FractionType::NormalFrac),
        map(tag("DisplayFrac"), |_| node::FractionType::DisplayFrac),
        map(tag("InlineFrac"), |_| node::FractionType::InlineFrac),
        map(tag("NoLineFrac"), |_| node::FractionType::NoLineFrac),
    ))(input)
}

// EFraction FractionType (Exp) (Exp)
fn parse_fraction(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EFraction")(output)?;
    (output, _) = multispace0(output)?;

    let (tmp, fraction_type) = parse_fraction_type(output)?;
    output = tmp;

    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;

    let (tmp, exp1) = parse_exp(output)?;
    output = tmp;

    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp2) = parse_exp(output)?;
    output = tmp;

    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EFraction(fraction_type, Box::new(exp1), Box::new(exp2))))
}

// ESqrt (Exp)
fn parse_sqrt(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ESqrt")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, node::Exp::ESqrt(Box::new(exp))))
}

#[test]
fn test_parse_super() {
    let test_case = "ESuper (ENumber \"1\") (ENumber \"2\")";
    assert_eq!(
        parse_super(test_case), 
        Ok(
            ("", 
            node::Exp::ESuper(Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string()))))
        ));
}

// ESuper (Exp) (Exp)
fn parse_super(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ESuper")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp1) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp2) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, node::Exp::ESuper(Box::new(exp1), Box::new(exp2))))
}

#[test]
fn test_parse_sub() {
    let test_case = "ESub (ENumber \"1\") (ENumber \"2\")";
    assert_eq!(
        parse_sub(test_case), 
        Ok(
            ("", 
            node::Exp::ESub(Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string()))))
        ));
}

// ESub (Exp) (Exp)
fn parse_sub(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ESub")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp1) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp2) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, node::Exp::ESub(Box::new(exp1), Box::new(exp2))))
}

#[test]
fn test_parse_subsup() {
    let test_case = "ESubsup (ENumber \"1\") (ENumber \"2\") (ENumber \"3\")";
    assert_eq!(
        parse_subsup(test_case), 
        Ok(
            ("", 
            node::Exp::ESubsup(Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string())), 
            Box::new(node::Exp::ENumber("3".to_string()))))
        ));
}

// ESubsup (Exp) (Exp) (Exp)
fn parse_subsup(input: &str) -> IResult<&str, node::Exp> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ESubsup")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp1) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp2) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, exp3) = parse_exp(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, node::Exp::ESubsup(Box::new(exp1), Box::new(exp2), Box::new(exp3))))
}

#[test]
fn test_parse_over() {
    let test_case = r#"
    EOver False (EIdentifier "\981") (ESymbol Accent "\771")
    "#;
    assert_eq!(
        parse_over(test_case.trim()), 
        Ok(
            ("", 
            node::Exp::EOver(false,
            Box::new(node::Exp::EIdentifier("\\981".to_string())),
            Box::new(node::Exp::ESymbol(node::TeXSymbolType::Accent, "\\771".to_string()))))
        )
    );
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
        map(tag("True"), |_| true),
        map(tag("true"), |_| true),
        map(tag("false"), |_| false),
        map(tag("False"), |_| false),
    ))(input)
}

// EOver false (Exp) (Exp)
fn parse_over(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EOver")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, bool) = parse_bool(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp1) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp2) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EOver(bool, Box::new(exp1), Box::new(exp2))))
}

// EUnder false (Exp) (Exp)
fn parse_under(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EUnder")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, bool) = parse_bool(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp1) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp2) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EUnder(bool, Box::new(exp1), Box::new(exp2))))
}

// EUnderover false (Exp) (Exp) (Exp)
fn parse_under_over(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EUnderover")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, bool) = parse_bool(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp1) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp2) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp3) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EUnderOver(bool, Box::new(exp1), Box::new(exp2), Box::new(exp3))))
}

#[test]
fn test_parse_phantom() {
    let test_case = "EPhantom (ENumber \"1\")";
    assert_eq!(
        parse_phantom(test_case), 
        Ok(
            ("", 
            node::Exp::EPhantom(Box::new(node::Exp::ENumber("1".to_string()))))
        ));
}

#[test]
fn test_parse_left() {
    let test_case = r#"
    Left "|"
    "#;
    println!("{:?}", parse_left(test_case));
}

// Left "\8722"
fn parse_left(input: &str) -> IResult<&str, node::InEDelimited> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("Left")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, text) = parse_quoted_string(output)?;
    output = tmp;
    Ok((output, node::InEDelimited::Left(text)))
}

#[test]
fn test_parse_right(){
    let test_case = r#"
    Right (EFraction NormalFrac (ENumber "2") (EIdentifier "x"))
    "#;
    let (output, exp) = parse_right(test_case).unwrap();
    assert_eq!(
        (output.trim(), exp), 
        ("", node::InEDelimited::Right(
            node::Exp::EFraction(
                node::FractionType::NormalFrac,
                Box::new(node::Exp::ENumber("2".to_string())),
                Box::new(node::Exp::EIdentifier("x".to_string()))
            )
        ))
    );


    let test_case = r#"
    Right (ENumber "5")
    "#;
    let (output, exp) = parse_right(test_case).unwrap();
    assert_eq!(
        (output.trim(), exp), 
        ("", node::InEDelimited::Right(node::Exp::ENumber("5".to_string())))
    );
}
// Right (Exp)
fn parse_right(input: &str) -> IResult<&str, node::InEDelimited> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("Right")(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::InEDelimited::Right(exp)))
}

// EPhantom (Exp)
fn parse_phantom(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EPhantom")(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EPhantom(Box::new(exp))))
}


#[test]
fn test_parse_boxed() {
    let test_case = "EBoxed (ENumber \"1\")";
    assert_eq!(
        parse_boxed(test_case), 
        Ok(
            ("", 
            node::Exp::EBoxed(Box::new(node::Exp::ENumber("1".to_string()))))
        ));
}

// EBoxed (Exp)
fn parse_boxed(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EBoxed")(output)?;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EBoxed(Box::new(exp))))
}

#[test]
fn test_parse_styled() {
    let test_case = "EStyled TextNormal [ENumber \"1\"]";
    assert_eq!(
        parse_styled(test_case), 
        Ok(
            ("", 
            node::Exp::EStyled(node::TextType::TextNormal,
            vec![node::Exp::ENumber("1".to_string())]))
        ));

    let test_case = r#"
    EStyled
        TextBoldScript
        [ EArray
            [ AlignLeft ]
            [ [ [ EText TextNormal "ABCDEFGHIJKLMNOPQRSTUVWXYZ" ] ]
            , [ [ EText TextNormal "abcdefghijklmnopqrstuvwxyz" ] ]
            ]
        ]
    "#;
    println!("{:?}", parse_styled(test_case));
}

// EStyled TextType [Exp]
fn parse_styled(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EStyled")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, text_type) = parse_text_type(output)?;
    output = tmp;

    (output, _) = multispace0(output)?;
    let (tmp, exp_list) = parse_exp_list(output)?;
    output = tmp;
    Ok((output, node::Exp::EStyled(text_type, exp_list)))
}

#[test]
fn test_parse_rational() {
    let test_case = "(1 % 2)";
    assert_eq!(
        parse_rational(test_case), 
        Ok(
            ("", 
            node::Rational{numerator: 1, denominator: 2})
        ));

    let test_case = "((-1) % 2)";

    assert_eq!(
        parse_rational(test_case), 
        Ok(
            ("", 
            node::Rational{numerator: -1, denominator: 2})
        ));
    let test_case = "(1 % (-2))";

    assert_eq!(
        parse_rational(test_case), 
        Ok(
            ("", 
            node::Rational{numerator: 1, denominator: -2})
        ));
}

#[test]
fn test_parse_i32() {
    let test_case = "12";
    assert_eq!(
        parse_i32(test_case), 
        Ok(
            ("", 
            12)
        ));

    let test_case = "-13";
    assert_eq!(
        parse_i32(test_case), 
        Ok(
            ("", 
            -13)
        ));
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, _) = multispace0(input)?;
    if input.starts_with('-'){
        let (input, _) = char('-')(input)?;
        let (input, _) = multispace0(input)?;
        let (input, i) = digit1(input)?;
        Ok((input, -i.parse::<i32>().unwrap()))
    }else{
        let (input, i) = digit1(input)?;
        Ok((input, i.parse::<i32>().unwrap()))
    }
}

// (numerator % denominator)
fn parse_rational(input: &str) -> IResult<&str, node::Rational> {
    let mut input = input;
    (input, _) = multispace0(input)?;
    (input, _) = char('(')(input)?;
    (input, _) = multispace0(input)?;
    
    let numerator : i32;
    let denominator : i32;
    let mut end_bracket = false;

    if input.starts_with('('){
        // remove the first '('
        (input, _) = char('(')(input)?;
        end_bracket = true;
    }

    (input, _) = multispace0(input)?;
    (input, numerator) = parse_i32(input)?;
    (input, _) = multispace0(input)?;
    if end_bracket {
        (input, _) = char(')')(input)?;
    }

    
    (input, _) = multispace0(input)?;
    (input, _) = char('%')(input)?;
    (input, _) = multispace0(input)?;

    end_bracket = false;
    if input.starts_with('('){
        // remove the first '('
        (input, _) = char('(')(input)?;
        end_bracket = true;
    }

    (input, _) = multispace0(input)?;
    (input, denominator) = parse_i32(input)?;
    (input, _) = multispace0(input)?;

    if end_bracket {
        (input, _) = char(')')(input)?;
    }

    (input, _) = char(')')(input)?;
    Ok((input, node::Rational{
        numerator: numerator,
        denominator: denominator,
    }))
}

#[test]
fn test_parse_scaled() {
    let test_case = "EScaled (9 % 5) (ESymbol Open \"|\")";
    assert_eq!(
        parse_scaled(test_case), 
        Ok(
            ("", 
            node::Exp::EScaled(
                node::Rational{numerator: 9, denominator: 5},
                Box::new(node::Exp::ESymbol(node::TeXSymbolType::Open, "|".to_string()))
            ))
        ));
}

// EScaled (Rational) (Exp)
fn parse_scaled(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("EScaled")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, rational) = parse_rational(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char('(')(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, exp) = parse_exp(output)?;
    output = tmp;
    (output, _) = multispace0(output)?;
    (output, _) = char(')')(output)?;
    Ok((output, node::Exp::EScaled(rational, Box::new(exp))))
}

#[test]
fn test_parse_space() {
    let test_case = "ESpace (1 % 9)";
    assert_eq!(
        parse_space(test_case), 
        Ok(
            ("", 
            node::Exp::ESpace(
                node::Rational{numerator: 1, denominator: 9},
            ))
        ));
}
// ESpace (1 % 9)
fn parse_space(input: &str) -> IResult<&str, node::Exp> {
    let mut output = input;
    (output, _) = multispace0(output)?;
    (output, _) = tag("ESpace")(output)?;
    (output, _) = multispace0(output)?;
    let (tmp, rational) = parse_rational(output)?;
    output = tmp;
    Ok((output, node::Exp::ESpace(rational)))
}

fn parse_alignment(input: &str) -> IResult<&str, node::Alignment> {
    alt((
        map(tag("AlignLeft"), |_| node::Alignment::AlignLeft),
        map(tag("AlignRight"), |_| node::Alignment::AlignRight),
        map(tag("AlignCenter"), |_| node::Alignment::AlignCenter),
    ))(input)
}

fn parse_array(input: &str) -> IResult<&str, node::Exp> {
    let mut input = input;
    (input, _) = multispace0(input)?;
    (input, _) = tag("EArray")(input)?;
    (input, _) = multispace0(input)?;
    
    // alignments

    (input, _) = tag("[")(input)?;
    let mut aligns = Vec::new();
    if !input.starts_with(']'){
        loop {
            (input, _) = multispace0(input)?;
            let (input_tmp, alignment) = parse_alignment(input)?;
            aligns.push(alignment);
            input = input_tmp;
            (input, _) = multispace0(input)?;
            if input.starts_with(']'){
                break;
            }
            (input, _) = char(',')(input)?;
        }
    }

    (input, _) = multispace0(input)?;
    (input, _) = tag("]")(input)?;
    (input, _) = multispace0(input)?;

    // rows
    (input, _) = tag("[")(input)?;
    let mut rows:Vec<Vec<Vec<node::Exp>>> = Vec::new();
    loop {
        (input, _) = multispace0(input)?;

        let mut row:Vec<Vec<node::Exp>> = Vec::new();
        (input, _) = tag("[")(input)?;
        loop {
            (input, _) = multispace0(input)?;
            let (input_tmp, exp) = parse_exp_list(input)?;
            row.push(exp);
            input = input_tmp;
            (input, _) = multispace0(input)?;
            if input.starts_with(']'){
                break;
            }
            (input, _) = char(',')(input)?;
        }
        (input, _) = multispace0(input)?;
        (input, _) = tag("]")(input)?;
        rows.push(row);

        (input, _) = multispace0(input)?;
        if input.starts_with(']'){
            break;
        }
        (input, _) = char(',')(input)?;
    }
    (input, _) = multispace0(input)?;
    (input, _) = tag("]")(input)?;
    (input, _) = multispace0(input)?;

    Ok((input, node::Exp::EArray(aligns, rows)))
}

#[test]
fn test_parse_array() {
    let test_case = r#"
    EArray
      [ AlignLeft
      , AlignRight
      ] 
      [ [ []
        , [ENumber "1"]
        , [ENumber "2"]
        ]
      , [ [EText TextNormal "num"]
        , [ENumber "3"]
        , [ENumber "4"]
        ]
      ]
    "#;
    let res = parse_array(test_case);
    println!("{:?}", res);


}


#[test]
fn test_all(){
    let case = r#"
    [ EArray
    [ AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    , AlignCenter
    ]
    [ [ [ EText TextNormal "C" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Pun "," ]
      , [ ESymbol Rel "<" ]
      , [ ESymbol Alpha "L" ]
      , [ ESymbol Ord "\\" ]
      , [ ESymbol Alpha "l" ]
      , [ ESymbol Op "|" ]
      ]
    , [ [ EText TextNormal "D" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Bin "-" ]
      , [ ESymbol Rel "=" ]
      , [ ESymbol Alpha "M" ]
      , [ ESymbol Close "]" ]
      , [ ESymbol Alpha "m" ]
      , [ ESymbol Close "}" ]
      ]
    , [ [ EText TextNormal "E" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Alpha "." ]
      , [ ESymbol Rel ">" ]
      , [ ESymbol Alpha "N" ]
      , [ ESymbol Ord "^" ]
      , [ ESymbol Alpha "n" ]
      , [ ESymbol Accent "~" ]
      ]
    , [ [ EText TextNormal "F" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Ord "*" ]
      , [ ESymbol Ord "/" ]
      , [ ESymbol Ord "?" ]
      , [ ESymbol Alpha "O" ]
      , [ ESymbol Ord "_" ]
      , [ ESymbol Alpha "o" ]
      , [ ESymbol Ord "@" ]
      ]
    ]
]
    "#;

    let res = parse_exp(case);
    println!("{:?}", res);
}