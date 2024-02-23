use lazy_static::lazy_static;
use ahash::AHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use crate::ast::node::{Alignment, Exp, InEDelimited, Rational, TeXSymbolType, TextType};
use crate::ast::to_tex_unicode::get_math_tex_many;

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
            let key = DIACRITICALS_TABLE.get(s)?;
            Some(key.to_string())
        },
        _ => {
            // 如果是多个字符, 则先转换为unicode码点, 再查表
            match parse_as_unicode_char(s) {
                Some(c) => {
                    let key = DIACRITICALS_TABLE.get(c.to_string().as_str())?;
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


lazy_static! {
    static ref DIACRITICALS_TABLE: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
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

#[test]
fn test_escape_latex(){
    let case = 'a';
    let res = escape_latex(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, None);

    let case = '#';
    let res = escape_latex(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, Some("\\#".to_string()));

    let case = '$';
    let res = escape_latex(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, Some("\\$".to_string()));

    let case = '\\';
    let res = escape_latex(case);
    println!("case: {:?}, res: {:?}", case, res);
    assert_eq!(res, Some("\\textbackslash".to_string()));

}

// 转义latex特殊字符:
// # -> \#
// $ -> \$
pub fn escape_latex(c: char) -> Option<String>{
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
        '~' => Some("\\textasciitilde".to_string()),
        '^' => Some("\\textasciicircum".to_string()),
        '\\' => Some("\\textbackslash".to_string()),
        '\u{200B}' => Some("\\!".to_string()),
        '\u{200A}' => Some("\\,".to_string()),
        '\u{2006}' => Some("\\,".to_string()),
        '\u{A0}' => Some("~".to_string()),
        '\u{2005}' => Some("\\:".to_string()),
        '\u{2004}' => Some("\\;".to_string()),
        '\u{2001}' => Some("\\quad".to_string()),
        '\u{2003}' => Some("\\quad".to_string()),
        '\u{2032}' => Some("'".to_string()),
        '\u{2033}' => Some("''".to_string()),
        '\u{2034}' => Some("'''".to_string()),
        '#' | '$' | '%' | '&' | '_' | '{' | '}' | ' ' => Some("\\".to_string() + &c.to_string()),
        _ => None
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

#[test]
fn test_get_general_frac(){
    let s = get_general_frac("[", "]");
    assert_eq!(s, "\\genfrac{[}{]}{0pt}{}");
}

// 获取通用的分数
pub fn get_general_frac(open: &str, close: &str) -> String{
    // \genfrac{left-delim}{right-delim}{thickness}{style}{numerator}{denominator}
    // \genfrac{左分隔符}{右分隔符}{厚度}{样式}{分子}{分母}
    // eg: \genfrac{[}{]}{0pt}{}{x}{y}
    let mut s = String::new();
    s.push_str("\\genfrac");
    s.push_str("{");
    s.push_str(open);
    s.push_str("}{");
    s.push_str(close);
    s.push_str("}{0pt}{}");
    s
}

// 输出alignments, 不带{}
// AlignLeft -> l, AlignRight -> r, AlignCenter -> c
pub fn get_alignments(aligns: &Vec<Alignment>) -> String{
    let mut res = String::with_capacity(aligns.len());
    for align in aligns{
        res.push_str(match align {
            Alignment::AlignLeft => {
                "l"
            },

            Alignment::AlignRight => {
                "r"
            },

            Alignment::AlignCenter => {
                "c"
            },
        });
    }
    res
}


// check if all exp is right
pub fn is_all_right(exp_list: &Vec<InEDelimited>) -> bool{
    for exp in exp_list{
        match exp {
            InEDelimited::Left(..) => {
                return false;
            },
            InEDelimited::Right(_) => {}
        }
    }
    return true;
}

// 把字符串的每一个字符转换为unicode escape
// 需要同时处理转义字符和utf8码点\d{4}
pub fn escape_text_as_tex(s: &str, envs: &HashMap<String, bool>) -> String{
    let (res, _) = get_math_tex_many(s, envs);
    return res
}

// check if all exp is standard height:
// Right(ENumber, EIdentifier, ESpace, ESymbol(Ord, Op, Bin, Rel, Pun))
pub fn is_all_standard_height(exp: &Vec<InEDelimited>) -> bool{
    for e in exp{
        match e {
            InEDelimited::Left(..) => {
                return false;
            },
            InEDelimited::Right(exp) => {
                match exp{
                    Exp::ENumber(..) => {},
                    Exp::EIdentifier(..) => {},
                    Exp::ESpace(..) => {},
                    Exp::ESymbol(TeXSymbolType::Ord, ..) => {},
                    Exp::ESymbol(TeXSymbolType::Op, ..) => {},
                    Exp::ESymbol(TeXSymbolType::Bin, ..) => {},
                    Exp::ESymbol(TeXSymbolType::Rel, ..) => {},
                    Exp::ESymbol(TeXSymbolType::Pun, ..) => {},
                    _ => {
                        return false;
                    }
                }
            },
        }
    }
    return true;
}


pub fn get_scaler_cmd(rational: &Rational) -> Option<String>{
    let need_width = rational.numerator as f64 / rational.denominator as f64;
    // 6/5 -> \big
    // 9/5 -> \Big
    // 12/5 -> \bigg
    // 15/5 -> \Bigg
    if need_width <= 1.2 {
        return Some("\\big".to_string());
    }else if need_width <= 1.8 {
        return Some("\\Big".to_string());
    }else if need_width <= 2.4 {
        return Some("\\bigg".to_string());
    }else if need_width <= 3.0 {
        return Some("\\Bigg".to_string());
    }
    return None;
}

pub fn get_diacritical_cmd(pos: &Position, s: &str) -> Option<String>{
    let cmd = get_diacriticals(s);

    match cmd {
        Some(cmd) => {
            if cmd == "\\overbracket" || cmd == "\\underbracket" {
                // -- We want to parse these but we can't represent them in LaTeX
                // unavailable :: [T.Text]
                // unavailable = ["\\overbracket", "\\underbracket"]
                return None;
            }

            let below = is_below(cmd.as_str());
            match pos{
                Position::Under => {
                    if below{
                        return Some(cmd);
                    }
                },
                Position::Over => {
                    if !below{
                        return Some(cmd);
                    }
                }
            }
        },
        None => {}
    }
    return None;
}
#[warn(unused_variables)]
pub fn get_style_latex_cmd(style: &TextType, _envs: &HashMap<String, bool>) -> String{
    // TODO: 处理环境, 有些环境可能不支持某些style, 如mathbfit
    // 现在仅仅将它转化为标准的LaTeX命令
    match style{
        &TextType::TextNormal => "\\mathrm".to_string(),
        &TextType::TextBold => "\\mathbf".to_string(),
        &TextType::TextItalic => "\\mathit".to_string(),
        &TextType::TextMonospace => "\\mathtt".to_string(),
        &TextType::TextBoldItalic => "\\mathbfit".to_string(),
        &TextType::TextSansSerif => "\\mathsf".to_string(),
        // &TextType::TextSansSerifBold => "\\mathbfsf".to_string(),
        &TextType::TextSansSerifBold => "\\mathbf".to_string(),
        // &TextType::TextSansSerifItalic => "\\mathbfsf".to_string(),
        &TextType::TextSansSerifItalic => "\\mathsf".to_string(),
        &TextType::TextSansSerifBoldItalic => "\\mathbfsfit".to_string(),
        &TextType::TextScript => "\\mathcal".to_string(),
        &TextType::TextFraktur => "\\mathfrak".to_string(),
        &TextType::TextDoubleStruck => "\\mathbb".to_string(),
        // &TextType::TextBoldFraktur => "\\mathbffrak".to_string(),
        &TextType::TextBoldFraktur => "\\mathfrak".to_string(),
        // &TextType::TextBoldScript => "\\mathbfscr".to_string(),
        &TextType::TextBoldScript => "\\mathcal".to_string(),
    }
}

// 获取\text的cmd, 有可能有多个cmd
// 第二个返回值是cmd的个数, 添加{}的个数
pub fn get_text_cmd(t: &TextType) -> (String, u8){
    match t{
        &TextType::TextNormal => ("\\text{".to_string(),1),
        &TextType::TextBold => ("\\textbf{".to_string(),1),
        &TextType::TextItalic => ("\\textit{".to_string(),1),
        &TextType::TextMonospace => ("\\texttt{".to_string(),1),
        &TextType::TextBoldItalic => ("\\textit{\\textbf{".to_string(),2),
        &TextType::TextSansSerif => ("\\textsf{".to_string(),1),
        &TextType::TextSansSerifBold => ("\\textbf{\\textsf{".to_string(),2),
        &TextType::TextSansSerifItalic => ("\\textit{\\textsf{".to_string(),2),
        &TextType::TextSansSerifBoldItalic => ("\\textbf{\\textit{\\textsf{".to_string(),3),
        _ => ("\\text{".to_string(),1),
    }
}

pub fn get_xarrow(e: &Exp) -> Option<String>{
    return match e {
        Exp::ESymbol(TeXSymbolType::Op, s) => {
            return if s == "\\8594" {
                Some("\\xrightarrow".to_string())
            } else if s == "\\8592" {
                Some("\\xleftarrow".to_string())
            } else {
                None
            }
        },
        _ => None,
    }
}

pub fn is_fancy(e: &Exp) -> bool{
    match e{
        &Exp::ESub(..) => true,
        &Exp::ESuper(..) => true,
        &Exp::ESubsup(..) => true,
        &Exp::EUnder(..) => true,
        &Exp::EOver(..) => true,
        &Exp::EUnderOver(..) => true,
        &Exp::ERoot(..) => true,
        &Exp::ESqrt(..) => true,
        &Exp::EPhantom(..) => true,
        _ => false,
    }
}

// 判断是否是RL序列:
// RL序列是指以AlignRight开头，以AlignLeft结尾，中间可以有任意多个AlignRight和AlignLeft
pub fn aligns_is_rlsequence(aligns: &Vec<Alignment>) -> bool{
    // isRLSequence :: [Alignment] -> Bool
    // isRLSequence [AlignRight, AlignLeft] = True
    // isRLSequence (AlignRight : AlignLeft : as) = isRLSequence as
    // isRLSequence _ = False
    return if aligns.len() % 2 == 0 {
        for align_pair in aligns.chunks(2) {
            if align_pair[0] != Alignment::AlignRight || align_pair[1] != Alignment::AlignLeft {
                return false;
            }
        }
        true
    } else {
        false
    }
}

// 判断是否是全部是AlignCenter, 这样的话可以使用matrix
pub fn aligns_is_all_center(aligns: &Vec<Alignment>) -> bool{
    for align in aligns{
        if align != &Alignment::AlignCenter{
            return false;
        }
    }
    return true;
}

// Esymbol Op 或者 EMathOperator
pub fn is_operator(e: &Exp) -> bool{
    match e{
        &Exp::ESymbol(TeXSymbolType::Op, ..) => true,
        &Exp::EMathOperator(..) => true,
        _ => false,
    }
}

pub enum FenceType{
    DLeft,
    DMiddle,
    DRight,
}

#[derive(PartialEq, Debug)]
pub enum Position{
    Under,
    Over,
}
