use lazy_static::lazy_static;
use ahash::AHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
#[test]
fn test_get_diacriticals(){
    let case = "\\8254";
    let res = get_diacriticals(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, Some("\\bar".to_string()));

    let case = "^";
    let res = get_diacriticals(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, Some("\\hat".to_string()));
}

// 转换uncode码点为对应的命令:
// ‾ -> \bar, ‾ = \u{203E} = \8254
pub fn get_diacriticals(s: &str) -> Option<String>{
    return match s.len() {
        1 => {
            // 如果是一个字符, 则直接查表
            let key = diacriticals_table.get(s)?;
            Some(key.to_string())
        },
        _ => {
            // 如果是多个字符, 则先转换为unicode码点, 再查表
            match parse_as_unicode_char(s) {
                Some(c) => {
                    let key = diacriticals_table.get(c.to_string().as_str())?;
                    Some(key.to_string())
                },
                None => {
                    return None;
                }
            }
        }
    }
}

pub fn is_below(s: &str) -> bool {
    // under = ["\\underbrace", "\\underline", "\\underbar", "\\underbracket"]
    s == "\\underbrace" || s == "\\underline" || s == "\\underbar" || s == "\\underbracket"
}

pub fn is_unavailable(s: &str) -> bool {
    // ["\\overbracket", "\\underbracket"]
    s == "\\overbracket" || s == "\\underbracket"
}


lazy_static! {
    static ref diacriticals_table: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let mut m :HashMap::<&'static str, &'static str, BuildHasherDefault<AHasher>> = HashMap::with_capacity_and_hasher(34, BuildHasherDefault::<AHasher>::default());
        /*
        // unicode码点对应的命令表, 如果相同则以最后一个为准
        [ ("\x00B4", "\\acute")
        , ("\x0301", "\\acute")
        , ("\x0060", "\\grave")
        , ("\x0300", "\\grave")
        , ("\x02D8", "\\breve")
        , ("\x0306", "\\breve")
        , ("\x02C7", "\\check")
        , ("\x030C", "\\check")
        , ("\x307", "\\dot")
        , ("\x308", "\\ddot")
        , ("\x20DB", "\\dddot")
        , ("\x20DC", "\\ddddot")
        , ("\x00B0", "\\mathring")
        , ("\x030A", "\\mathring")
        , ("\x20D7", "\\vec")
        , ("\x20D7", "\\overrightarrow")
        , ("\x20D6", "\\overleftarrow")
        , ("\x005E", "\\hat")
        , ("\x02C6", "\\widehat")
        , ("\x0302", "\\widehat")
        , ("\x02DC", "\\widetilde")
        , ("\x0303", "\\tilde")
        , ("\x0303", "\\widetilde")
        , ("\x0304", "\\bar")
        , ("\x203E", "\\bar")
        , ("\x23DE", "\\overbrace")
        , ("\x23B4", "\\overbracket") -- Only availible in mathtools
        , ("\x00AF", "\\overline")
        , ("\x0305", "\\overline")
        , ("\x23DF", "\\underbrace")
        , ("\x23B5", "\\underbracket") -- mathtools
        , ("\x0332", "\\underline")
        , ("_", "\\underline")
        , ("\x0333", "\\underbar")
        ]
        */
        m.insert("\u{00B4}", "\\acute");
        m.insert("\u{0301}", "\\acute");
        m.insert("\u{0060}", "\\grave");
        m.insert("\u{0300}", "\\grave");
        m.insert("\u{02D8}", "\\breve");
        m.insert("\u{0306}", "\\breve");
        m.insert("\u{02C7}", "\\check");
        m.insert("\u{030C}", "\\check");
        m.insert("\u{307}", "\\dot");
        m.insert("\u{308}", "\\ddot");
        m.insert("\u{20DB}", "\\dddot");
        m.insert("\u{20DC}", "\\ddddot");
        m.insert("\u{00B0}", "\\mathring");
        m.insert("\u{030A}", "\\mathring");
        m.insert("\u{20D7}", "\\overrightarrow");
        m.insert("\u{20D6}", "\\overleftarrow");
        m.insert("\u{005E}", "\\hat");
        m.insert("\u{02C6}", "\\widehat");
        m.insert("\u{0302}", "\\widehat");
        m.insert("\u{02DC}", "\\widetilde");
        m.insert("\u{0303}", "\\widetilde");
        m.insert("\u{0304}", "\\bar");
        m.insert("\u{203E}", "\\bar");
        m.insert("\u{23DE}", "\\overbrace");
        m.insert("\u{23B4}", "\\overbracket"); // Only availible in mathtools
        m.insert("\u{00AF}", "\\overline");
        m.insert("\u{0305}", "\\overline");
        m.insert("\u{23DF}", "\\underbrace");
        m.insert("\u{23B5}", "\\underbracket"); // mathtools
        m.insert("\u{0332}", "\\underline");
        m.insert("_", "\\underline");
        m.insert("\u{0333}", "\\underbar");
        m
    };
}

#[test]
fn test_parse_unicode_escape(){
    println!("{:?}", parse_as_unicode_char("\\65024"));
}

// 转换为unicode对应的字符:
// "\\8481" -> "℡"
pub fn parse_as_unicode_char(s: &str) -> Option<char> {
    let code_point = u32::from_str_radix(s.trim_start_matches('\\'), 10).ok()?;
    char::from_u32(code_point)
}

// 转义latex特殊字符:
// # -> \#
// $ -> \$
pub fn escape_latex(c: char) -> String{
    // case c of
    // '~'   -> ControlSeq "\\textasciitilde"
    // '^'   -> Literal "\\textasciicircum"
    // '\\'  -> ControlSeq "\\textbackslash"
    // '\x200B' -> Literal "\\!"
    // '\x200A' -> Literal "\\,"
    // '\x2006' -> Literal "\\,"
    // '\xA0'   -> Literal "~"
    // '\x2005' -> Literal "\\:"
    // '\x2004' -> Literal "\\;"
    // '\x2001' -> ControlSeq "\\quad"
    // '\x2003' -> ControlSeq "\\quad"
    // '\x2032' -> Literal "'"
    // '\x2033' -> Literal "''"
    // '\x2034' -> Literal "'''"
    // _ | T.any (== c) "#$%&_{} " -> Literal ("\\" <> T.singleton c)
    //     | otherwise -> Token c
    match c {
        '~' => "\\textasciitilde".to_string(),
        '^' => "\\textasciicircum".to_string(),
        '\\' => "\\textbackslash".to_string(),
        '\u{200B}' => "\\!".to_string(),
        '\u{200A}' => "\\,".to_string(),
        '\u{2006}' => "\\,".to_string(),
        '\u{A0}' => "~".to_string(),
        '\u{2005}' => "\\:".to_string(),
        '\u{2004}' => "\\;".to_string(),
        '\u{2001}' => "\\quad".to_string(),
        '\u{2003}' => "\\quad".to_string(),
        '\u{2032}' => "'".to_string(),
        '\u{2033}' => "''".to_string(),
        '\u{2034}' => "'''".to_string(),
        '#' | '$' | '%' | '&' | '_' | '{' | '}' | ' ' => "\\".to_string() + &c.to_string(),
        _ => c.to_string()
    }
}

pub fn is_mathoperator(s: &str) -> bool {
    // operators :: M.Map Exp T.Text
    // operators = M.fromList
    // [ (EMathOperator "arccos", "\\arccos")
    // , (EMathOperator "arcsin", "\\arcsin")
    // , (EMathOperator "arctan", "\\arctan")
    // , (EMathOperator "arg", "\\arg")
    // , (EMathOperator "cos", "\\cos")
    // , (EMathOperator "cosh", "\\cosh")
    // , (EMathOperator "cot", "\\cot")
    // , (EMathOperator "coth", "\\coth")
    // , (EMathOperator "csc", "\\csc")
    // , (EMathOperator "deg", "\\deg")
    // , (EMathOperator "det", "\\det")
    // , (EMathOperator "dim", "\\dim")
    // , (EMathOperator "exp", "\\exp")
    // , (EMathOperator "gcd", "\\gcd")
    // , (EMathOperator "hom", "\\hom")
    // , (EMathOperator "inf", "\\inf")
    // , (EMathOperator "ker", "\\ker")
    // , (EMathOperator "lg", "\\lg")
    // , (EMathOperator "lim", "\\lim")
    // , (EMathOperator "liminf", "\\liminf")
    // , (EMathOperator "limsup", "\\limsup")
    // , (EMathOperator "ln", "\\ln")
    // , (EMathOperator "log", "\\log")
    // , (EMathOperator "max", "\\max")
    // , (EMathOperator "min", "\\min")
    // , (EMathOperator "Pr", "\\Pr")
    // , (EMathOperator "sec", "\\sec")
    // , (EMathOperator "sin", "\\sin")
    // , (EMathOperator "sinh", "\\sinh")
    // , (EMathOperator "sup", "\\sup")
    // , (EMathOperator "tan", "\\tan")
    // , (EMathOperator "tanh", "\\tanh") ]
    match s {
        "arccos" | "arcsin" | "arctan" | "arg" | "cos" | "cosh" | "cot" | "coth" | "csc" | "deg" | "det" | "dim" | "exp" | "gcd" | "hom" | "inf" | "ker" | "lg" | "lim" | "liminf" | "limsup" | "ln" | "log" | "max" | "min" | "Pr" | "sec" | "sin" | "sinh" | "sup" | "tan" | "tanh" => true,
        _ => false
    }
}