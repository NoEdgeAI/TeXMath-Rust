use std::collections::HashMap;

pub fn escape_symbol(symbol: &str, envs: &HashMap<String, bool>) -> String{
    let mut s = String::new();
    match symbol{
        "\\8722" => {
            s.push_str("-");
        },
        "\\177" => {
            s.push_str("\\pm");
        },
        "\\8747" => {
            s.push_str("\\int");
        },
        _ => {
            s.push_str(symbol);
        }
    }
    s
}