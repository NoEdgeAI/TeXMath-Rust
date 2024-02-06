use std::{collections::HashMap, hash::BuildHasherDefault};
use lazy_static::lazy_static;
use ahash::{AHasher};

#[test]
fn test_parse_unicode_escape(){
    println!("{:?}", parse_unicode_escape("\\65024"));
}
// parse unicode escape: "\\8481" -> "℡"
pub(crate) fn parse_unicode_escape(s: &str) -> Option<char> {
    // if s == "\\65024"{
    //     return Some('︀');
    // }
    let digits = s.trim_start_matches('\\');
    // 将十进制字符串转换为 u32
    let code_point = u32::from_str_radix(digits, 10).ok()?;

    // 将 u32 转换为一个 char
    char::from_u32(code_point)
}

#[derive(Debug)]
enum CharType {
    Unicode(String),
    Escape(char),
    Normal(char),
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

fn escape_char(c: char) -> String{
    match c {
        'n' => "\\n".to_string(),
        't' => "\\t".to_string(),
        'r' => "\\r".to_string(),
        '"' => "\\f".to_string(),
        '\'' => "\\v".to_string(),
        '\\' => "\\\\".to_string(),
        _ => panic!("unknown escape char: {}", c)
    }
}

#[test]
fn test_get_math_tex_many(){
    let s = "a\\n\\t\\r\\8722\\177\\8747, hello, i am \\65024";
    let envs = HashMap::new();
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    println!("{:?}", res.as_bytes());
    assert_eq!(res, "a\\n\\t\\r-\\pm\\int, hello, i am \u{fe00}");

    let s = "Consider the equation\\160";
    let want = "Consider the equation~";
    let res = get_math_tex_many(s, &envs);
    dbg!(&res);
    assert_eq!(res, want);
}
// 转换字符串为tex格式
pub fn get_math_tex_many(s: &str, envs: &HashMap<String, bool>) -> String{
    let mut res = String::new();
    let chars = spilt_str(s);
    for c in chars {
        match c {
            CharType::Unicode(num) => {
                res.push_str(&to_unicode(&num, envs));
            },
            CharType::Escape(c) => {
                res.push_str(&escape_char(c));
            },
            CharType::Normal(c) => {
                res.push(c);
            }
        }
    }
    res
}

#[test]
fn test_escapse_symbol(){
    assert_eq!(to_unicode("\\8722", &HashMap::new()), "-");
    assert_eq!(to_unicode("\\177", &HashMap::new()), "\\pm");
    assert_eq!(to_unicode("\\8747", &HashMap::new()), "\\int");
}

fn to_unicode(symbol: &str, envs: &HashMap<String, bool>) -> String{
    // try base symbol
    if let Some(base) = tex_table.get(("base_".to_owned() + symbol).as_str()) {
        return base.to_string();
    }else{
        // try other envs
        for (env, _) in envs {
            if let Some(base) = tex_table.get((env.to_owned() + "_" + symbol).as_str()) {
                return base.to_string();
            }
        }
    }

    // try unicode escape
    if let Some(c) = parse_unicode_escape(symbol) {
        return c.to_string();
    }

    // TODO: 输出mathcal, mathbb这类特殊字体
    // TODO: escapse unicode
    return symbol.to_string(); // not found, return original symbol
}

struct KeyVal {
    key: String,
    val: String,
}

fn read_csv(path: &str) -> Result<Vec<KeyVal>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut key_vals = Vec::new();
    for result in reader.records() {
        let record = result?;
        let key = record.get(0).ok_or("Missing key")?.to_string();
        let val = record.get(1).ok_or("Missing value")?.to_string();
        key_vals.push(KeyVal { key, val });
    }
    Ok(key_vals)
}

lazy_static! {
    static ref tex_table: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
        // TODO: use config file
        let path = r#"E:\Code\Rust\texmath\src\ast\to_tex_unicode_table.csv"#;
        let key_vals = read_csv(path).expect("not found csv file, please check path");

        let mut m :HashMap::<&'static str, &'static str, BuildHasherDefault<AHasher>> = HashMap::with_capacity_and_hasher(key_vals.len(), BuildHasherDefault::<AHasher>::default());
        for key_val in key_vals {
            let key = Box::leak(key_val.key.into_boxed_str());
            let val = Box::leak(key_val.val.into_boxed_str());
            m.insert(key, val);
        }
        m
    };
}
