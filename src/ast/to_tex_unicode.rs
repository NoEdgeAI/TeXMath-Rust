use std::{collections::HashMap, hash::BuildHasherDefault};
use lazy_static::lazy_static;
use ahash::AHasher;
use crate::config;
use super::{node, shared::{escape_latex, parse_as_unicode_char}};


fn spilt_as_char(s: &str) -> Vec<char>{
    let mut res = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c == '\\' {
            // 以\开头的情况有:
            // 1. \n \t \r等转义字符 -> Escape
            // 2. \d{1~5} unicode码点 -> Unicode
            // 3. \" \\ 等引号内转义字符
            // TODO: 可能会出现i+1越界的情况, 主要是\后面没有字符的情况, 实际上是非法的
            let next = s.chars().nth(i + 1).unwrap();
            if next.is_ascii_digit() {
                let mut j = i + 1;
                while j < s.len() && s.chars().nth(j).unwrap().is_ascii_digit() {
                    j += 1;
                }
                // \d{1~5} -> \12345
                let num = s.get(i + 1..j).unwrap().parse::<u32>().unwrap();
                if let Some(unicode) = std::char::from_u32(num) {
                    res.push(unicode);
                }else{
                    panic!("invalid unicode: {:?}", num);
                }
                i = j;
            }else{
                if next == 'n' || next == 't' || next == 'r' {
                    // \n \t \r \f \v
                    match next {
                        'n' => res.push('\n'),
                        't' => res.push('\t'),
                        'r' => res.push('\r'),
                        _ => panic!("unknown escape char: {:?}", next)
                    }
                }else{
                    // 引号内的转义字符
                    res.push(next);
                }
                i += 2;
            }
        }else{
            res.push(c);
            i += 1;
        }
    }
    res
}
#[test]
fn test_escapse_text(){
    let s = r#"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_""#;
    let res = escapse_text(s);
    println!("{:?}", res);
}
// 把文本中的\1234直接转为unicode
pub fn escapse_text(s: &str) -> String{
    let mut res = String::new();
    let chars = spilt_as_char(s);
    for c in chars {
        if let Some(escaped) = escape_latex(c) {
            if escaped == "\\ "{
                res.push_str(" ");
            }else{
                res.push_str(&escaped);
            }
        }else{
            res.push(c);
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
    println!("{:?}", res.0.as_bytes());
    assert_eq!(res.0, "a\n\t\r-\\pm\\int,test");

    let s = "C\\160\\8203";
    let want = "C~\\hspace{0pt}";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res.0, want);

    let s = "\\8202";
    let want = "\\,";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res.0, want);

    let s = "\\8203";
    let want = "\\hspace{0pt}";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res.0, want);
}

// 转换字符串为tex输出
// 1. unicode+env -> tex命令
// 2. 转义字符 -> 转义输出
// 3. \d{1~5} -> \12345 unicode转换
// return: (tex, tokens数量), \alpha -> (\alpha, 1)
pub fn get_math_tex_many(s: &str, envs: &HashMap<String, bool>) -> (String, usize){
    let mut res = String::new();

    let chars = spilt_as_char(s);
    let num = chars.len();
    for c in chars {
        if c == '\u{fe00}' {
            // -- we ignore 65024 VARIATION SELECTOR 1 to avoid putting it
            //     -- literally in the output ; it is used in mathml output.
            //     charToLaTeXString _ '\65024' = Just []
            continue;
        }else if c== '\u{2061}' || c == '\u{2062}' || c == '\u{2063}' || c == '\u{2064}' {
            // writeExp (ESymbol Ord (T.unpack -> [c]))  -- do not render "invisible operators"
            //   | c `elem` ['\x2061'..'\x2064'] = return () -- see
            continue;
        }

        if let Some(tex_cmd) = lookup_tex_cmd_table(&c, envs) {
            res.push_str(&tex_cmd.val);

            // [Accent, Rad, TOver, TUnder] -> Categories which require braces
            if tex_cmd.category == "Accent" || tex_cmd.category == "Rad" || tex_cmd.category == "TOver" || tex_cmd.category == "TUnder" {
                res.push_str("{}");
            }
        }else if let Some(tex_cmd) = look_rev_text_unicode_table(&c) {
            res.push_str(&tex_cmd);
        }else if let Some(tex_cmd) = escape_latex(c) {
            res.push_str(&tex_cmd);
        }else {
            res.push(c);
        }

    }
    (res, num)
}

#[test]
fn test_lookup_tex_cmd_table(){
    // assert_eq!(lookup_tex_cmd_table("\\8722", &HashMap::new()), Some(
    //     tex_cmd_val{
    //         category: "Bin".to_string(),
    //         val: "-".to_string(),
    //     }
    // ));
    // assert_eq!(lookup_tex_cmd_table("\\177", &HashMap::new()), Some(
    //     tex_cmd_val{
    //         category: "Ord".to_string(),
    //         val: "\\pm".to_string(),
    //     }
    // ));
    // assert_eq!(lookup_tex_cmd_table("\\8747", &HashMap::new()), Some(
    //     tex_cmd_val{
    //         category: "Op".to_string(),
    //         val: "\\int".to_string(),
    //     }
    // ));
    // assert_eq!(lookup_tex_cmd_table("\\8594", &HashMap::new()), Some(
    //     tex_cmd_val{
    //         category: "Rel".to_string(),
    //         val: "\\rightarrow".to_string(),
    //     }
    // ));
}

// 查表, 转换unicode码点为tex命令
// \120432 -> \mathtt{A}; env = base
fn lookup_tex_cmd_table(c: &char, envs: &HashMap<String, bool>) -> Option<TexCmdVal>{
    // try base symbol
    if let Some(base) = TEX_TABLE.get(("base_".to_string() + c.to_string().as_str()).as_str()) {
        let res = TexCmdVal{
            category: base.category.to_string(),
            val: base.val.to_string(),
        };
        return Some(res);
    }else{
        // try other envs
        for (env, _) in envs {
            if let Some(base) = TEX_TABLE.get((env.to_string() + "_" + c.to_string().as_str()).as_str()) {
                let res = TexCmdVal{
                    category: base.category.to_string(),
                    val: base.val.to_string(),
                };
                return Some(res);
            }
        }
    }
    None
}

#[test]
fn test_look_rev_text_unicode_table(){
    let case = parse_as_unicode_char("\\8488").unwrap();
    let res = look_rev_text_unicode_table(&case);
    assert_eq!(res, Some("\\mathfrak{Z}".to_string()));
}
fn look_rev_text_unicode_table(unicode: &char) -> Option<String>{
    REV_TEXT_UNICODE_TABLE.get(unicode.to_string().as_str()).map(|v| v.to_string())
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

#[derive(PartialEq, Debug)]
struct TexCmdKey{
    env: String,
    c: char,
}
#[derive(PartialEq, Debug)]
struct TexCmdVal{
    pub category: String,
    pub val: String,
}

lazy_static! {
    static ref TEX_TABLE: HashMap<&'static str, &'static TexCmdVal, BuildHasherDefault<AHasher>> = {
        let prefix = config::get_config().table_dir.as_str();
        let path = prefix.to_string() + "/tex_cmd_table.csv";
        let mut key_vals = csv::Reader::from_path(path).expect("read records err for tex_cmd_table.csv");

        let mut m :HashMap<&'static str, &'static TexCmdVal, BuildHasherDefault<AHasher>> = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
        for result in key_vals.records() {
            let record = result.expect("Could not read record");
            let unicode_str = record.get(1).expect("Missing unicode");
            let unicode;
            if unicode_str.len() != 1 {
                // \开头的unicode码点, 转换为char
                let c = parse_as_unicode_char(unicode_str).expect("parse unicode err");
                unicode = c;
            }else{
                // 非\开头的unicode码点, 直接转换为char
                unicode = unicode_str.chars().next().expect("parse unicode err");
            }

            // dbg!(unicode.clone());

            // env_c -> tex命令
            let key = Box::leak(Box::new(
                format!("{}_{}", record.get(0).expect("Missing env"), unicode)
            ));
            let val = Box::leak(Box::new(TexCmdVal{
                category: record.get(2).expect("Missing category").to_string(),
                val: record.get(3).expect("Missing val").to_string(),
            }));

            // TODO: tex_cmd_table中有些字符顺序是反的, 所以只添加第一个
            if m.contains_key(key.as_str()) {
                continue;
            }
            m.insert(key, val);
        }
        m
    };

    // text type + text -> unicode
    static ref TEXT_UNICODE_TABLE: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let prefix = config::get_config().table_dir.as_str();
        let path = prefix.to_string() + "/text_unicode_table.csv";
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
    static ref REV_TEXT_UNICODE_TABLE: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        let prefix = config::get_config().table_dir.as_str();
        let path = prefix.to_string() + "/text_unicode_table.csv";
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
#[test]
fn test_is_delimiters(){
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let s = "\u{27e8}";
    let res = is_delimiters(&s, &envs);
    assert_eq!(res, true);

    let s = "|";
    let res = is_delimiters(&s, &envs);
    assert_eq!(res, true);
}

pub fn is_delimiters(s: &str, envs: &HashMap<String, bool>) -> bool{
    if s.len() == 0 {
        return false;
    }
    let mut c = s.chars().next().unwrap();
    if s.len() > 1 && s.starts_with("\\") {
        // 可能是unicode码点
        if let Some(unicode) = parse_as_unicode_char(s) {
            c = unicode;
        }else{
            // \arrowvert 这样的命令
            return false;
        }
    }
    // TODO: 对envs的每个环境都生成一个列表, 再判断s是否在列表中, 这里直接查Open, Close可行吗?
    let base_cmds = vec!['.', '(', ')', '[', ']', '|', '\u{2016}', '{', '}'
                         , '\u{2309}', '\u{2308}', '\u{2329}', '\u{232A}'
                         , '\u{230B}', '\u{230A}', '\u{231C}', '\u{231D}'];
    // 这里仅仅判断了最基本的情况
    if base_cmds.contains(&c){
        return true;
    }else if let Some(cmd) = lookup_tex_cmd_table(&c, envs) {
        if cmd.category == "Open" || cmd.category == "Close" {
            return true;
        }
    }
    return false;
}
