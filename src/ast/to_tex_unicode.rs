use std::{collections::HashMap, hash::BuildHasherDefault};
use lazy_static::lazy_static;
use ahash::AHasher;
use super::{node, shared::{escape_latex, parse_as_unicode_char}};

#[derive(Debug)]
enum CharType {
    Unicode(String), // \d{1~5} -> \12345
    Escape(char), // \n \t \r 等
    Normal(char), // 普通字符, a b c 1 2 3 等
}

#[test]
fn test_spilt_str(){
    let case = "a\\n\\t\\r\\f\\v\\8722\\177\\8747";
    let res = spilt_str(case);
    println!("{:?}", res);
}
// 把长串的text转换为码点:
// 码点分3种:
// 1. unicode码点, \d{1~5} -> \12345
// 2. 转义字符, \n \t \r ... -> 转义输出
// 3. 普通字符, a b c -> 直接输出
fn spilt_str(s: &str) -> Vec<CharType>{
    let mut res = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c == '\\' {
            let next = s.chars().nth(i + 1).unwrap();
            if next.is_ascii_digit() {
                let mut j = i + 1;
                while j < s.len() && s.chars().nth(j).unwrap().is_ascii_digit() {
                    j += 1;
                }
                // \d{1~5} -> \12345
                res.push(CharType::Unicode(s[i ..j].to_string()));
                i = j;
            }else{
                res.push(CharType::Escape(next));
                i += 2;
            }
        }else{
            res.push(CharType::Normal(c));
            i += 1;
        }
    }
    res
}

#[test]
fn test_get_math_tex_many(){
    let s = "a\\n\\t\\r\\8722\\177\\8747,test\\65024";
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    println!("{:?}", res.as_bytes());
    assert_eq!(res, "a\\n\\t\\r-\\pm\\int,test\u{fe00}");

    let s = "C\\160\\8203";
    let want = "C~\\hspace{0pt}";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res, want);

    let s = "\\8202";
    let want = "\\,";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res, want);
}

// 转换字符串为tex输出
// 1. unicode+env -> tex命令
// 2. 转义字符 -> 转义输出
// 3. \d{1~5} -> \12345 unicode转换
pub fn get_math_tex_many(s: &str, envs: &HashMap<String, bool>) -> String{
    let mut res = String::new();
    let chars = spilt_str(s);
    for c in chars {
        match c {
            CharType::Unicode(num) => {
                if let Some(cmd) = lookup_tex_cmd_table(num.as_str(), envs) {
                    res.push_str(cmd.as_str());
                }else {
                    if let Some(unicode) = parse_as_unicode_char(num.as_str()) {
                        if let Some(cmd) = look_rev_text_unicode_table(&unicode.to_string()) {
                            res.push_str(cmd.as_str());
                        }else{
                            res.push_str(escape_latex(unicode).as_str());
                        }
                    }else{
                        panic!("unknown unicode: {:?}", num);
                    }
                }
            },
            CharType::Escape(c) => {
                res.push_str(
                    &match c {
                    'n' => "\\n".to_string(),
                    't' => "\\t".to_string(),
                    'r' => "\\r".to_string(),
                    _ => c.to_string()
                });
            },
            CharType::Normal(c) => {
                res.push_str(&escape_latex(c));
            }
        }
    }
    res
}

#[test]
fn test_lookup_tex_cmd_table(){
    assert_eq!(lookup_tex_cmd_table("\\8722", &HashMap::new()), Some("-".to_string()));
    assert_eq!(lookup_tex_cmd_table("\\177", &HashMap::new()), Some("\\pm".to_string()));
    assert_eq!(lookup_tex_cmd_table("\\8747", &HashMap::new()), Some("\\int".to_string()));
    assert_eq!(lookup_tex_cmd_table("\\8594", &HashMap::new()), Some("\\rightarrow".to_string()));
}

// 查表, 转换unicode码点为tex命令
// \120432 -> \mathtt{A}; env = base
fn lookup_tex_cmd_table(symbol: &str, envs: &HashMap<String, bool>) -> Option<String>{
    // try base symbol
    if let Some(base) = tex_table.get(("base_".to_string() + symbol).as_str()) {
        return Some(base.to_string());
    }else{
        // try other envs
        for (env, _) in envs {
            if let Some(base) = tex_table.get((env.to_string() + "_" + symbol).as_str()) {
                return Some(base.to_string());
            }
        }
    }
    None
}
#[test]
fn test_look_text_unicode_table(){
    // "TextFraktur","Z","\8488"
    let t = node::TextType::TextFraktur;
    let s = "Z".to_string();
    let res = look_text_unicode_table(&t, &s);
    assert_eq!(res, Some("\\8488".to_string()));
}

// "TextFraktur","Z" -> "\8488"
fn look_text_unicode_table(t: &node::TextType, s: &String) -> Option<String>{
    let key = text_type_to_str(t) + "_" + s;
    text_unicode_table.get(key.as_str()).map(|v| v.to_string())
}

#[test]
fn test_look_rev_text_unicode_table(){
    let case = parse_as_unicode_char("\\8488").unwrap();
    let res = look_rev_text_unicode_table(&case.to_string());
    assert_eq!(res, Some("\\mathfrak{Z}".to_string()));
}
fn look_rev_text_unicode_table(unicode: &String) -> Option<String>{
    rev_text_unicode_table.get(unicode.as_str()).map(|v| v.to_string())
}
fn text_type_to_str(t: &node::TextType) -> String{
    match t {
        node::TextType::TextNormal => "TextNormal",
        node::TextType::TextBoldItalic => "TextBoldItalic",
        node::TextType::TextBoldScript => "TextBoldScript",
        node::TextType::TextBoldFraktur => "TextBoldFraktur",
        node::TextType::TextBold => "TextBold",
        node::TextType::TextItalic => "TextItalic",
        node::TextType::TextMonospace => "TextMonospace",
        node::TextType::TextSansSerifItalic => "TextSansSerifItalic",
        node::TextType::TextSansSerifBoldItalic => "TextSansSerifBoldItalic",
        node::TextType::TextSansSerifBold => "TextSansSerifBold",
        node::TextType::TextSansSerif => "TextSansSerif",
        node::TextType::TextDoubleStruck => "TextDoubleStruck",
        node::TextType::TextScript => "TextScript",
        node::TextType::TextFraktur => "TextFraktur",
    }.to_string()
}

fn str_to_text_type(s: &str) -> node::TextType{
    match s {
        "TextNormal" => node::TextType::TextNormal,
        "TextBoldItalic" => node::TextType::TextBoldItalic,
        "TextBoldScript" => node::TextType::TextBoldScript,
        "TextBoldFraktur" => node::TextType::TextBoldFraktur,
        "TextBold" => node::TextType::TextBold,
        "TextItalic" => node::TextType::TextItalic,
        "TextMonospace" => node::TextType::TextMonospace,
        "TextSansSerifItalic" => node::TextType::TextSansSerifItalic,
        "TextSansSerifBoldItalic" => node::TextType::TextSansSerifBoldItalic,
        "TextSansSerifBold" => node::TextType::TextSansSerifBold,
        "TextSansSerif" => node::TextType::TextSansSerif,
        "TextDoubleStruck" => node::TextType::TextDoubleStruck,
        "TextScript" => node::TextType::TextScript,
        "TextFraktur" => node::TextType::TextFraktur,
        _ => panic!("unknown text type: {:?}", s)
    }
}

fn text_type_cmd(t: &node::TextType) -> String{
    // --TextType to (MathML, LaTeX)
    // textTypes :: [(TextType, (T.Text, T.Text))]
    // textTypes =
    //     [ ( TextNormal       , ("normal", "\\mathrm"))
    //         , ( TextBold         , ("bold", "\\mathbf"))
    //         , ( TextItalic       , ("italic","\\mathit"))
    //         , ( TextMonospace    , ("monospace","\\mathtt"))
    //         , ( TextSansSerif    , ("sans-serif","\\mathsf"))
    //         , ( TextDoubleStruck , ("double-struck","\\mathbb"))
    //         , ( TextScript       , ("script","\\mathcal"))
    //         , ( TextFraktur      , ("fraktur","\\mathfrak"))
    //         , ( TextBoldItalic          , ("bold-italic","\\mathbfit"))
    //         , ( TextSansSerifBold       , ("bold-sans-serif","\\mathbfsfup"))
    //         , ( TextSansSerifBoldItalic , ("sans-serif-bold-italic","\\mathbfsfit"))
    //         , ( TextBoldScript          , ("bold-script","\\mathbfscr"))
    //         , ( TextBoldFraktur         , ("bold-fraktur","\\mathbffrak"))
    //         , ( TextSansSerifItalic     , ("sans-serif-italic","\\mathsfit")) ]
    match t {
        node::TextType::TextNormal => "\\mathrm",
        node::TextType::TextBold => "\\mathbf",
        node::TextType::TextItalic => "\\mathit",
        node::TextType::TextMonospace => "\\mathtt",
        node::TextType::TextSansSerif => "\\mathsf",
        node::TextType::TextDoubleStruck => "\\mathbb",
        node::TextType::TextScript => "\\mathcal",
        node::TextType::TextFraktur => "\\mathfrak",
        node::TextType::TextBoldItalic => "\\mathbfit",
        node::TextType::TextSansSerifBold => "\\mathbfsfup",
        node::TextType::TextSansSerifBoldItalic => "\\mathbfsfit",
        node::TextType::TextBoldScript => "\\mathbfscr",
        node::TextType::TextBoldFraktur => "\\mathbffrak",
        node::TextType::TextSansSerifItalic => "\\mathsfit",
    }.to_string()
}

lazy_static! {
    static ref tex_table: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let path = r#"E:\Code\Rust\texmath\src\ast\tables\tex_cmd_table.csv"#;
        let mut key_vals = csv::Reader::from_path(path).expect("read records err for tex_cmd_table.csv");

        let mut m :HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
        for result in key_vals.records() {
            // TODO: tex_cmd_table中有些字符顺序不对, 需要重新读取调整
            let record = result.expect("Could not read record");
            let env = record.get(0).expect("Missing env");
            let c = record.get(1).expect("Missing char");
            let val = Box::leak(Box::new(record.get(2).expect("Missing val").to_string()));
            let key = Box::leak(Box::new(format!("{}_{}", env, c)));
            m.insert(key, val);
        }
        m
    };

    // text type + text -> unicode
    static ref text_unicode_table: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let path = r#"E:\Code\Rust\texmath\src\ast\tables\text_unicode_table.csv"#;
        let mut reader = csv::Reader::from_path(path).expect("read records err for text_unicode_table.csv");
        let mut m :HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
        for result in reader.records() {
            let record = result.expect("Could not read record");
            let text_type_str = record.get(0).expect("Missing text_type");

            let text = record.get(1).expect("Missing text");
            let unicode = record.get(2).expect("Missing Unicode");

            let key = Box::leak(Box::new(format!("{}_{}", text_type_str, text)));
            let val = Box::leak(Box::new(unicode.to_string()));
            m.insert(key, val);
        }
        m
    };

    // unicode码点对应的命令表, 如果相同则以最后一个为准
    // 如: \u{xxxx} -> \mathbb{A}
    static ref rev_text_unicode_table: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let path = r#"E:\Code\Rust\texmath\src\ast\tables\text_unicode_table.csv"#;
        let mut reader = csv::Reader::from_path(path).expect("read records err for text_unicode_table.csv");
        let mut m :HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
        for result in reader.records() {
            let record = result.expect("Could not read record");
            let text_type_str = record.get(0).expect("Missing text_type");
            let text_type = str_to_text_type(text_type_str);
            let text_cmd = text_type_cmd(&text_type);

            let text = record.get(1).expect("Missing text");
            let unicode_parsed_text = if text.starts_with("\\") {
                match parse_as_unicode_char(text) {
                    Some(c) => c.to_string(),
                    None => panic!("parse unicode err")
                }
            }else{
                text.to_string()
            };

            let origin_unicode = record.get(2).expect("Missing Unicode");
            let parsed_unicode = parse_as_unicode_char(origin_unicode).expect("parse unicode err");

            let val = Box::leak(Box::new(text_cmd + "{" + &unicode_parsed_text + "}"));

            let key = Box::leak(Box::new(parsed_unicode.to_string()));
            m.insert(key, val);

            // println!("key: {:?}, val: {:?}", key, val);
        }
        m
    };
}
