use std::{collections::HashMap, hash::BuildHasherDefault};
use lazy_static::lazy_static;
use ahash::{AHasher};

#[test]
fn test_escapse_symbol(){
    assert_eq!(escape_symbol_unicode("\\8722", &HashMap::new()), "-");
    assert_eq!(escape_symbol_unicode("\\177", &HashMap::new()), "\\pm");
    assert_eq!(escape_symbol_unicode("\\8747", &HashMap::new()), "\\int");
}
#[test]
fn test_parse_unicode_escape(){
    println!("{:?}", parse_unicode_escape("\\65024"));
}
// parse unicode escape: "\\8481" -> "℡"
fn parse_unicode_escape(s: &str) -> Option<char> {
    // if s == "\\65024"{
    //     return Some('︀');
    // }
    let digits = s.trim_start_matches('\\');
    // 将十进制字符串转换为 u32
    let code_point = u32::from_str_radix(digits, 10).ok()?;

    // 将 u32 转换为一个 char
    char::from_u32(code_point)
}

pub fn escape_symbol_unicode(symbol: &str, envs: &HashMap<String, bool>) -> String{
    // try base symbol
    if let Some(base) = SYMBOLS.get(("base_".to_owned() + symbol).as_str()) {
        return base.to_string();
    }else{
        // try other envs
        for (env, _) in envs {
            if let Some(base) = SYMBOLS.get((env.to_owned() + "_" + symbol).as_str()) {
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
    static ref SYMBOLS: HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>> = {
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
