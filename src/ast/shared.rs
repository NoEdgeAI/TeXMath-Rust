use lazy_static::lazy_static;
use ahash::{AHasher};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use super::to_tex_unicode::parse_unicode_escape;
/*
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
#[test]
fn test_get_diacriticals(){
    let case = "\\8254";
    let res = get_diacriticals(case);
    print!("{:?}", res);
    assert_eq!(res, Some("\\bar".to_string()));
}
pub fn get_diacriticals(s: &str) -> Option<String>{
    let spilted: Vec<char> = s.chars().collect();
    if spilted.len() >= 2 && spilted[0] == '\\' && spilted[spilted.len() - 1].is_ascii_digit(){
        // \d, 转换为unicode
        let res = parse_unicode_escape(s);
        if let Some(v) = res{
            return get_diacriticals(&v.to_string());
        }
    }
    if let Some(v) = diacriticals_table.get(s){
        Some(v.to_string())
    }else{
        None
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
        // m.insert("\u{20D7}", "\\vec"); // TODO: why two same key?
        m.insert("\u{20D7}", "\\overrightarrow");
        m.insert("\u{20D6}", "\\overleftarrow");
        m.insert("\u{005E}", "\\hat");
        m.insert("\u{02C6}", "\\widehat");
        m.insert("\u{0302}", "\\widehat");
        m.insert("\u{02DC}", "\\widetilde");
        // m.insert("\u{0303}", "\\tilde");
        m.insert("\u{0303}", "\\widetilde");
        m.insert("\u{0304}", "\\bar");
        m.insert("\u{203E}", "\\bar");
        m.insert("\u{23DE}", "\\overbrace");
        // m.insert("\u{23B4}", "\\overbracket"); // Only availible in mathtools
        m.insert("\u{00AF}", "\\overline");
        m.insert("\u{0305}", "\\overline");
        m.insert("\u{23DF}", "\\underbrace");
        // m.insert("\u{23B5}", "\\underbracket"); // mathtools
        m.insert("\u{0332}", "\\underline");
        m.insert("_", "\\underline");
        m.insert("\u{0333}", "\\underbar");
        m
    };
}