use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use super::{node, shared};
use super::to_tex_unicode;
use super::node::{Exp, Alignment, ArrayLines, TextType, TeXSymbolType, FractionType, InEDelimited, Rational};

/*
 * 加空格的情况:
 * 1. \alphax -> \alpha x: ESymbol的转义和字母之间加空格
 * 2. \fracxy -> \frac xy: \frac的转义和两个变量之间加空格
 */

// // Tex的控制序列
// enum TexSeqType{
//     Control, // 控制序列, 如\frac, \sqrt: \fracxy -> \frac xy
//     Literal, // 符号: \alpha, \beta, \gamma: \alphax -> \alpha x
//     Start, // 开始: 表示初始状态
// }

struct TexWriterContext {
    tex: String, // 输出的TeX: 由于需要递归, 所以需要一个全局变量, 用于存储输出的TeX, 不使用String的原因是为了避免频繁的内存分配
    envs: HashMap<String, bool>,
    last_control: bool, // 对于\frac, \alpha等没有{}的控制序列, 需要加空格才能接下一个字母, 用于表示上一个是不是控制符号且没有其他分隔符
}

#[test]
fn test_tex_writer(){
    let case = r#"
    [ESub (EIdentifier "\981") (EIdentifier "n")]"#;
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let exp = super::ast_reader::read_ast(case).unwrap();
    // dbg!(&exp);
    let tex = write_tex_with_env(exp, &envs);
    println!("{:?}", tex);
}

#[test]
fn test_judge_by_texmath(){
    let test_tex = r#"
f(x) = \begin{cases}
1 &  - 1 \le x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let right_tex = r#"
f(x) = \begin{cases}
1 & - 1 \leq x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let (flag, res) = judge_by_texmath(right_tex.to_string(), test_tex.to_string());
    assert_eq!(flag, true);
    println!("res: \n{}", res);
}
pub fn judge_by_texmath(right_tex: String, test_tex: String) -> (bool, String){
    let json = json!(
        {
            "display": false,
            "from": "tex",
            "to": "tex",
            "text": test_tex
        }
    );
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:3000/convert")
        .json(&json).send().unwrap();
    // println!("status: {}", res.status());
    // println!("headers: {:#?}", res.headers());
    if res.status() != 200{
        println!("status: {}", res.status());
        println!("headers: {:#?}", res.headers());
        return (false, "".to_string())
    }
    let body = res.text().unwrap();
    let body = body.trim().to_string().replace("\r", "");
    let right_tex = right_tex.trim().to_string().replace("\r", "");
    // println!("A: {:?}", body.as_bytes());
    // println!("B: {:?}", right_tex.as_bytes());
    if body == right_tex{
        return (true, body);
    }
    return (false, body);
}
#[test]
fn test_text_writer_file(){
    let path = "ast";
    let file_content = fs::read_to_string(path).unwrap();
    let mut native = String::new();
    let mut o_tex = String::new();
    // 提取<<< native 和 >>> tex之间的内容
    if file_content.find("<<< native").is_none() || file_content.find(">>> tex").is_none(){
        panic!("<<< native or >>> tex not found");
    }else{
        let start = file_content.find("<<< native").unwrap();
        let end = file_content.find(">>> tex").unwrap();
        native = file_content[start + "<<< native".len()..end].to_string();
        o_tex = file_content[end + ">>> tex".len()..].to_string().trim().to_string();
    }

    let exp = super::ast_reader::read_ast(&native).unwrap();
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let tex = write_tex_with_env(exp, &envs).unwrap().trim().to_string();
    
    let f = fs::File::create("./output").unwrap();
    let mut f = std::io::BufWriter::new(f);
    let (same, texmath) = judge_by_texmath(o_tex.clone(), tex.clone());
    println!("A: {:?}", tex.as_bytes());
    println!("B: {:?}", o_tex.as_bytes());
    println!("C: {:?}", texmath.as_bytes());

    println!("same: {}", same);

    f.write("same:".as_bytes()).unwrap();
    f.write(same.to_string().as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();

    f.write("ast:\n".as_bytes()).unwrap();
    f.write(native.as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();

    f.write("tex:\n".as_bytes()).unwrap();
    f.write(tex.as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();

    f.write("expect:\n".as_bytes()).unwrap();
    f.write(o_tex.as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();

    f.write("texmath:\n".as_bytes()).unwrap();
    f.write(texmath.as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();
}
// 把Exp转换为TeX, 带上环境
pub fn write_tex_with_env(exps: Vec<Exp>, envs: &HashMap<String, bool>) -> Result<String, String>{
    let twc = &mut TexWriterContext {
        tex: String::new(),
        envs: envs.clone(),
        last_control: false,
    };
    for exp in &exps {
        write_exp(twc, exp)?;
    }
    Ok(twc.tex.clone())
}

#[test]
fn test_get_general_frac(){
    let s = get_general_frac("[", "]");
    assert_eq!(s, "\\genfrac{[}{]}{0pt}{}");
}

// 获取通用的分数
fn get_general_frac(open: &str, close: &str) -> String{
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
fn get_alignments(aligns: &Vec<Alignment>) -> String{
    let mut res = String::with_capacity(aligns.len());
    for align in aligns{
        res.push_str(&align.to_str());
    }
    res
}
enum FenceType{
    DLeft,
    DMiddle,
    DRight,
}

#[derive(PartialEq, Debug)]
enum Position{
    Under,
    Over,
}

// 保证输出一对{}且不重复
// 但如果Exp是EGrouped, 直接调用write_tex会导致输出两对{}, 所以需要特殊处理
fn write_grouped_exp(c: &mut TexWriterContext, exp: &Exp) -> Result<(), String>{
   return  match exp {
        Exp::EGrouped(exp_list) => {
            c.tex.push_str("{");
            for e in exp_list{
                write_exp(c, e)?;
            }
            c.tex.push_str("}");
            Ok(())
        },
        _ => {
            c.tex.push_str("{");
            write_exp(c, exp)?;
            c.tex.push_str("}");
            Ok(())
        }
    }
}
// 输出array table
// name = "array" or "matrix"...
fn write_array_table(c: &mut TexWriterContext, name: &str, aligns: &Vec<Alignment>, rows: &Vec<ArrayLines>) -> Result<(), String>{
    // \begin{xxx}
    // \begin{array}{ccc}

    c.tex.push_str("\\begin{");
    c.tex.push_str(name);
    c.tex.push_str("}");
    // if has aligns
    if aligns.len() > 0 {
        c.tex.push_str("{");
        c.tex.push_str(get_alignments(aligns).as_str());
        c.tex.push_str("}");
    }
    c.tex.push_str("\n");

    // array rows
    match rows.len(){
        0 => {},
        1 => {
        },
        _ => {
            for row in rows{
                for ele in row{
                    // write arrayline:
                    match ele.len() {
                        0 => {},
                        1 => {
                            write_exp(c, &ele[0])?;
                        },
                        _ => {
                            for e in ele{
                                write_exp(c, e)?;
                            }
                        }
                    }

                    // 最后一个元素不需要输出&
                    if ele == &row[row.len() - 1]{
                        continue;
                    }
                    // 元素之间需要加上 &
                    c.tex.push_str(" & ");
                }

                if row == &rows[rows.len() - 1]{
                    c.tex.push_str("\n");
                    continue; // 最后一行不需要输出\\, 只需要换行, 后面加上\end{name}
                }
                c.tex.push_str(" ");
                c.tex.push_str("\\\\");
                c.tex.push_str("\n");
            }
        }
    }

    c.tex.push_str("\\end{");
    c.tex.push_str(name);
    c.tex.push_str("}");
    Ok(())
}

// 当Delimited只有一个Right元素且里面是EArray时调用
// Delimited open close [Right (EArray [AlignCenter] [[[x]],[[y]]])]
fn delimited_write_right_array(c: &mut TexWriterContext, open: &String, close: &String, aligns: &Vec<Alignment>, rows: &Vec<ArrayLines>) -> Result<(), String> {
    Ok(
        match (c.envs["amsmath"], open.as_str(), close.as_str()) {
            (true, "{", "") => {
                if aligns.len() == 2 && aligns[0] == Alignment::AlignLeft && aligns[1] == Alignment::AlignLeft {
                    // \begin{cases} \end{cases}
                    write_array_table(c, "cases", &Vec::<Alignment>::new(), rows)?;
                }
            }
            (true, "(", ")") => {
                if aligns_is_all_center(aligns) {
                    // \begin{pmatrix} \end{pmatrix}
                    write_array_table(c, "pmatrix", &Vec::<Alignment>::new(), rows)?;
                }
            }
            (true, "[", "]") => {
                if aligns_is_all_center(aligns) {
                    // \begin{bmatrix} \end{bmatrix}
                    write_array_table(c, "bmatrix", &Vec::<Alignment>::new(), rows)?;
                }
            }
            (true, "{", "}") => {
                if aligns_is_all_center(aligns) {
                    // \begin{Bmatrix} \end{Bmatrix}
                    write_array_table(c, "Bmatrix", &Vec::<Alignment>::new(), rows)?;
                }
            }
            // 读取进来的AST确实是这样的, 但是这里的open和close是unicode码点, 需不需要先escaped一遍做统一处理?
            // 例如说在READ AST的时候就把open和close转换为unicode码点
            // TODO: EDelimited 码点转义
            (true, "\\8739", "\\8739") => {
                if aligns_is_all_center(aligns) {
                    // \begin{vmatrix} \end{vmatrix}
                    write_array_table(c, "vmatrix", &Vec::<Alignment>::new(), rows)?;
                }
            }
            (true, "\\8741", "\\8741") => {
                if aligns_is_all_center(aligns) {
                    // \begin{Vmatrix} \end{Vmatrix}
                    write_array_table(c, "Vmatrix", &Vec::<Alignment>::new(), rows)?;
                }
            }
            _ => {
                delimited_write_delim(c, FenceType::DLeft, open);
                write_array_table(c, "array", aligns, rows)?;
                delimited_write_delim(c, FenceType::DRight, close);
                // delimited_write_delim(c, FenceType::DLeft, &open);
                // // TODO: write array is ?
                // write_exp(c, exp)?;
                // delimited_write_delim(c, FenceType::DRight, &close);
            },
        }
    )
}

fn write_binom(c: &mut TexWriterContext, cmd: &str, e1: &Exp, e2: &Exp) -> Result<(), String>{
    // \binom{a}{b}
    // TODO: write binom
    panic!("write_binom not implemented");
    Ok(())
}

// 处理 EDelimited open close [Right (EFraction NoLineFrac e1 e2)]
fn delimited_fraction_noline(c: &mut TexWriterContext, left: &String, right: &String, frac_exp1: &Exp, frac_exp2: &Exp) -> Result<(), String> {
    Ok(match (left.as_str(), right.as_str()) {
        ("(", ")") => {
            // \choose: 类似于二项
            write_binom(c, "\\choose", frac_exp1, frac_exp2)?;
        },
        ("[", "]") => {
            // \\brack
            write_binom(c, "\\brack", frac_exp1, frac_exp2)?;
        },
        ("{", "}") => {
            // \\brace
            write_binom(c, "\\brace", frac_exp1, frac_exp2)?;
        },
        ("\u{27E8}", "\u{27E9}") => {
            // \\bangle
            write_binom(c, "\\bangle", frac_exp1, frac_exp2)?;
        },
        _ => {
            // others:
            // writeExp (EDelimited open close [Right (EArray [AlignCenter]
            //     [[[x]],[[y]]])])

            delimited_write_right_array(c, left, right,
                                        &vec![Alignment::AlignCenter],
                                        &vec![
                                            vec![vec![frac_exp1.clone()]],
                                            vec![vec![frac_exp2.clone()]]
                                        ])?;
        }
    })
}

fn delimited_write_delim(c: &mut TexWriterContext, ft: FenceType, delim: &str){
    let tex_delim = get_tex_math_many(delim, &c.envs);
    let valid = is_delimiters(delim, &c.envs); // 界定符号是否有效
    let null_lim = get_tex_math_many(".", &c.envs); // TODO: 空的界定符号

    let delim_cmd = match valid {
        true => tex_delim.clone(),
        false => null_lim,
    }; // 如果有效则使用tex_delim, 否则使用null_lim(空的界定符号)

    match ft {
        FenceType::DLeft => {
            // valid: \left(
            // invalid: \left. tex
            c.tex.push_str("\\left");
            c.tex.push_str(&delim_cmd);
            c.tex.push_str(" ");
            if !valid {
                c.tex.push_str(&tex_delim);
            }
        },
        FenceType::DMiddle => {
            if valid{
                c.tex.push_str(" ");
                c.tex.push_str("\\middle");
                c.tex.push_str(&delim_cmd);
                c.tex.push_str(" ");
            }else{
                c.tex.push_str(&tex_delim);
            }
        },
        FenceType::DRight => {
            c.tex.push_str(" ");
            c.tex.push_str("\\right");
            c.tex.push_str(&delim_cmd);
            if !valid {
                c.tex.push_str(&tex_delim);
            }
        },
    }
}
fn delimited_write_general_exp(c: &mut TexWriterContext, open: &String, close: &String, exp_list: &Vec<InEDelimited>) -> Result<(), String>{
//     writeExp (EDelimited open close es)
//   | all isStandardHeight es
//   , open == "(" || open == "[" || open == "|"
//   , close == ")" || close == "]" || close == "|"
//   , all isRight es
//   = do
//     getTeXMathM open >>= tell
//     mapM_ (either (writeDelim DMiddle) writeExp) es
//     getTeXMathM close >>= tell
//  where
//   isStandardHeight (Right (EIdentifier{})) = True
//   isStandardHeight (Right (ENumber{})) = True
//   isStandardHeight (Right (ESpace{})) = True
//   isStandardHeight (Right (ESymbol ty _)) = ty elem` [Ord, Op, Bin, Rel, Pun]
//   isStandardHeight _ = False
    let is_open_close =
        match (open.as_str(), close.as_str()){
            ("(", ")") => {
                true
            },
            ("[", "]") => {
                true
            },
            ("{", "}") => {
                true
            },
            _ => {
                false
            }
        };

    let is_right = is_all_right(exp_list);
    let is_standard_height = is_all_standard_height(exp_list);
    return if is_open_close && is_right && is_standard_height {
        c.tex.push_str(&get_tex_math_many(open, &c.envs));
        // mapM_ (either (writeDelim DMiddle) writeExp) es
        for exp in exp_list {
            match exp {
                InEDelimited::Left(delim) => {
                    delimited_write_delim(c, FenceType::DMiddle, delim);
                },
                InEDelimited::Right(exp) => {
                    write_exp(c, exp)?;
                }
            }
        }
        c.tex.push_str(&get_tex_math_many(close, &c.envs));
        Ok(())
    } else {
        // writeExp (EDelimited open close es) =  do
        // writeDelim DLeft open
        // mapM_ (either (writeDelim DMiddle) writeExp) es
        // writeDelim DRight close
        delimited_write_delim(c, FenceType::DLeft, open);
        for exp in exp_list {
            match exp {
                InEDelimited::Left(delim) => {
                    delimited_write_delim(c, FenceType::DMiddle, delim);
                },
                InEDelimited::Right(exp) => {
                    write_exp(c, exp)?;
                }
            }
        }
        delimited_write_delim(c, FenceType::DRight, close);
        Ok(())
    }
}

fn write_script(c: &mut TexWriterContext, p: &Position, convertible: &bool, b: &Exp, e1: &Exp) -> Result<(), String>{
    // TODO: write script

    let dia_cmd = match e1{
        Exp::ESymbol(t, s) => {
            if t == &TeXSymbolType::Accent || t == &TeXSymbolType:: TOver || t == &TeXSymbolType::TUnder {
                get_diacritical_cmd(p, s)
            }else{
                None
            }
        },
        _ => {
            None
        }
    };

    if let Some(cmd) = dia_cmd {
        c.tex.push_str(&cmd);
        write_grouped_exp(c, b)?;
    }else{
        if is_operator(b){
            if is_fancy(b){
                write_grouped_exp(c, b)?;
            }else{
                // TODO: 可能要增加convertible对write_tex的影响
                if *convertible{
                    write_exp(c, b)?;
                }else{
                    c.tex.push_str("\\limits");
                }
                c.tex.push_str("_");
                // TODO: check_substack
                // check_substack(res, e1, envs);
                write_grouped_exp(c, e1)?;
            }
        }
    }
    Ok(())
}

// 在underover中其中一个是accent时调用
fn write_underover_accent(c: &mut TexWriterContext, exp: &Exp) -> bool{
    // (EUnderover convertible b e1@(ESymbol Accent _) e2) -> (EUnder convertible (EOver False b e2) e1)
    // (EUnderover convertible b e1 e2@(ESymbol Accent _)) -> (EOver convertible (EUnder False b e1) e2)

    return match exp {
        Exp::EUnderOver(convertible,b,e1,e2) => {
            if let Exp::ESymbol(TeXSymbolType::Accent,_) = **e1 {
                // e1是accent
                let new_under_base = Exp::EUnder(
                    false,
                    (*b).clone(),
                    (*e2).clone()
                );
                let new_under = Exp::EUnder(
                    *convertible,
                    Box::new(new_under_base),
                    (*e1).clone()
                );
                write_exp(c, &new_under);
                return true;
            }else if let Exp::ESymbol(TeXSymbolType::Accent,_) = **e2 {
                // e2是accent
                let new_over_base = Exp::EOver(
                    false,
                    (*b).clone(),
                    (*e1).clone()
                );
                let new_over = Exp::EOver(
                    *convertible,
                    Box::new(new_over_base),
                    (*e2).clone()
                );
                write_exp(c, &new_over);
                return true;
            }
            false
        },
        _ => {
            false
        }
    }
}

fn check_substack(c: &mut TexWriterContext, e:&Exp){
    match e{
        Exp::EArray(aligns, rows) => {
            panic!("check_substack not implemented");
        },
        _ => {
            write_exp(c, e);
        }
    }
}



// check if all exp is right
fn is_all_right(exp_list: &Vec<InEDelimited>) -> bool{
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
fn get_tex_math_many(s: &str, envs: &HashMap<String, bool>) -> String{
    to_tex_unicode::get_math_tex_many(s, envs)
}

// check if all exp is standard height:
// Right(ENumber, EIdentifier, ESpace, ESymbol(Ord, Op, Bin, Rel, Pun))
fn is_all_standard_height(exp: &Vec<InEDelimited>) -> bool{
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

fn is_delimiters(s: &str, envs: &HashMap<String, bool>) -> bool{
    let cmds = vec![ ".", "(", ")", "[", "]", "|", "\x2016", "{", "}"
                     , "\u{2309}", "\u{2308}", "\u{2329}", "\u{232A}"
                     , "\u{230B}", "\u{230A}", "\u{231C}", "\u{231D}"];
    // TODO: 对envs的每个环境都生成一个列表, 再判断s是否在列表中
    // 这里仅仅判断了最基本的情况
    if cmds.contains(&s){
        return true;
    }
    return false;
}

fn get_scaler_cmd(rational: &Rational) -> Option<String>{
    // TODO: get scaler cmd
    panic!("get_scaler_cmd not implemented");
}
// 将\\ 转换为空格
fn fix_space(s: &str) -> String{
    if s == "\\ "{
        return " ".to_string();
    }
    return s.to_string();
}

fn get_diacritical_cmd(pos: &Position, s: &str) -> Option<String>{
    let cmd = shared::get_diacriticals(s);
    match cmd {
        Some(cmd) => {
            if shared::is_below(cmd.as_str()) {
                return None
            }
            let below = shared::is_below(cmd.as_str());
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

fn get_style_latex_cmd(style: &TextType, envs: &HashMap<String, bool>) -> String{
    // TODO: 处理环境, 有些环境可能不支持某些style, 如mathbfit
    match style{
        &TextType::TextNormal => "\\mathrm".to_string(),
        &TextType::TextBold => "\\mathbf".to_string(),
        &TextType::TextItalic => "\\mathit".to_string(),
        &TextType::TextMonospace => "\\mathtt".to_string(),
        &TextType::TextBoldItalic => "\\mathbfit".to_string(),
        &TextType::TextSansSerif => "\\mathsf".to_string(),
        &TextType::TextSansSerifBold => "\\mathbfsf".to_string(),
        &TextType::TextSansSerifItalic => "\\mathbfsf".to_string(),
        &TextType::TextSansSerifBoldItalic => "\\mathbfsfit".to_string(),
        &TextType::TextScript => "\\mathcal".to_string(),
        &TextType::TextFraktur => "\\mathfrak".to_string(),
        &TextType::TextDoubleStruck => "\\mathbb".to_string(),
        &TextType::TextBoldFraktur => "\\mathbffrak".to_string(),
        &TextType::TextBoldScript => "\\mathbfscr".to_string(),
    }
}

// 获取\text的cmd, 有可能有多个cmd
// 第二个返回值是cmd的个数, 添加{}的个数
fn get_text_cmd(t: &TextType) -> (String, u8){
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

fn get_xarrow(e: &Exp) -> Option<String>{
    return match e {
        Exp::ESymbol(TeXSymbolType::Op, s) => {
            return if s == "\u{2192}" {
                Some("\\xrightarrow".to_string())
            } else if s == "\u{2190}" {
                Some("\\xleftarrow".to_string())
            } else {
                None
            }
        },
        _ => None,
    }
}

// TODO: what is fancy
fn is_fancy(e: &Exp) -> bool{
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
fn aligns_is_rlsequence(aligns: &Vec<Alignment>) -> bool{
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
fn aligns_is_all_center(aligns: &Vec<Alignment>) -> bool{
    for align in aligns{
        if align != &Alignment::AlignCenter{
            return false;
        }
    }
    return true;
}

// Esymbol Op 或者 EMathOperator
fn is_operator(e: &Exp) -> bool{
    match e{
        &Exp::ESymbol(TeXSymbolType::Op, ..) => true,
        &Exp::EMathOperator(..) => true,
        _ => false,
    }
}

pub fn to_tex(exps: Vec<Exp>) -> Result<(),String> {
    let mut env = HashMap::<String, bool>::new();
    env.insert("amsmath".to_string(), true);
    env.insert("amssymb".to_string(), true);
    let tw = &mut TexWriterContext {
        tex: String::new(),
        envs: env,
        last_control: false,
    };
    for exp in &exps {
        write_exp(tw, exp)?;
    }
    Ok(())
}


fn write_exp(c: &mut TexWriterContext, exp: &Exp) -> Result<(), String>{
    match exp{
        Exp::ENumber(n) => {
            c.last_control = false;
            c.tex.push_str(n);
        },

        Exp::EBoxed(exp) => {
            c.last_control = false;
            if c.envs["amsmath"]{
                c.tex.push_str("\\boxed{");
                write_exp(c, exp)?;
                c.tex.push_str("}");
            }else{
                write_exp(c, exp)?;
            }
        },

        Exp::EGrouped(exp_list) => {
            c.last_control = false;
            // 如果只有一个元素, 则不需要{}
            if exp_list.len() == 1{
                write_exp(c, &exp_list[0])?;
            }else{
                c.tex.push_str("{");
                for exp in exp_list{
                    write_exp(c, exp)?;
                }
                c.tex.push_str("}");
            }

        },

        Exp::EDelimited(left, right, exp_list) => {

            c.last_control = false;
            // EDelimited open close [Right (EFraction NoLineFrac e1 e2)]
            if exp_list.len() == 1{
                match &exp_list[0] {
                    InEDelimited::Right(Exp::EFraction(FractionType::NoLineFrac, e1, e2)) => {
                        return delimited_fraction_noline(c, left, right, e1, e2);
                    },
                    InEDelimited::Right(Exp::EArray(aligns, rows)) => {
                        // Delimited open close [Right (EArray [AlignCenter] [[[x]],[[y]]])]
                        return delimited_write_right_array(c, left, right, aligns, rows);
                    },
                    _ => {
                        // go to below
                    }
                }
            }
            return delimited_write_general_exp(c, left, right, exp_list);
        },

        Exp::ESymbol(symbol_type, symbol) => {
            // writeExp (ESymbol Ord (T.unpack -> [c]))  -- do not render "invisible operators"
            //   | c `elem` ['\x2061'..'\x2064'] = return () -- see 3.2.5.5 of mathml spec

            if symbol_type == &TeXSymbolType::Ord && symbol.len() == 1{
                let c = symbol.chars().next().unwrap();
                if c >= '\u{2061}' && c <= '\u{2064}'{
                    return Ok(());
                }
            }


            let escaped = get_tex_math_many(&symbol, &c.envs);
            // 如果是Bin, Rel则需要添加一个空格
            if *symbol_type == TeXSymbolType::Bin || *symbol_type == TeXSymbolType::Rel{
                // 如果已经以空格结尾, 则不需要再添加空格
                if c.tex.chars().last().unwrap() != ' '{
                    c.tex.push_str(" ");
                }
                // c.tex.push_str(" ");
            }

            c.tex.push_str(&escaped);
            // TODO: symbol escape
            // if symbol.len() > 1 && (symbol_type == &node::TeXSymbolType::Bin || symbol_type == &node::TeXSymbolType::Rel || symbol_type == &node::TeXSymbolType::Op) {
            //     s.push_str("\\math");
            //     s.push_str(symbol_type.to_show().as_str());
            //     s.push_str("{");

            //     s.push_str("\\text{");
            //     s.push_str(&escaped);
            //     s.push_str("}");

            //     s.push_str("}");
            // }

            // 如果是Bin, Rel则需要添加一个空格
            if *symbol_type == TeXSymbolType::Bin || *symbol_type == TeXSymbolType::Rel{
                c.tex.push_str(" ");
            }

            // \开头, 最后一个是字母 -> 可能需要空格
            if escaped.starts_with("\\") && escaped.chars().last().unwrap().is_alphabetic(){
                c.last_control = true;
            }
        },

        // ok
        Exp::ESpace(rational) => {
            c.last_control = false;
            let width = rational.numerator as f32 / rational.denominator as f32 * 18.0;
            let width = width.floor() as i32;
            match width {
                -3 => {
                    c.tex.push_str("\\!");
                },
                0 => {}
                3 => {
                    c.tex.push_str("\\, ");
                },
                4 => {
                    // use: \\  \\: \\>
                    c.tex.push_str("\\ ");
                },
                5 => {
                    c.tex.push_str("\\;");
                },
                18 => {
                    c.tex.push_str("\\quad");
                    c.last_control = true;
                    return Ok(());
                },
                36 => {
                    c.tex.push_str("\\qquad ");
                    c.last_control = true;
                    return Ok(());
                },
                n => {
                    if c.envs["amsmath"]{
                        c.tex.push_str("\\mspace{");
                        c.tex.push_str(&n.to_string());
                        c.tex.push_str("mu}");
                    }else{
                        c.tex.push_str("\\mskip{");
                        c.tex.push_str(&n.to_string());
                        c.tex.push_str("mu}");
                    }
                }
            }

        },

        Exp::EIdentifier(identifier) => {
            // 为了防止连续的标识符被合并, 需要在标识符之间添加空格, 如:
            // \alphax -> \alpha x
            let escaped = get_tex_math_many(&identifier, &c.envs);
            if c.last_control && c.tex.chars().last().unwrap().is_alphabetic() && escaped.chars().next().unwrap().is_alphabetic(){
                c.tex.push_str(" ");
                c.last_control = false;
            }

            let escaped = get_tex_math_many(&identifier, &c.envs);
            if escaped.len() == 0{
                return Ok(());
            }
            c.tex.push_str(&escaped);

            // escape 开头为\\, c.tex结尾为字母 -> 为控制符 \xxx
            if escaped.starts_with("\\") && escaped.chars().last().unwrap().is_alphabetic(){
                c.last_control = true;
            }
        },

        Exp::EMathOperator(math_operator) => {
            // TODO: more precise MathOperator
            c.tex.push_str("\\");
            c.tex.push_str(&math_operator);
            // TODO: space_control
        },

        Exp::ESub(exp1, exp2) => {
            if is_fancy(exp1){
                c.tex.push_str("{");
                write_exp(c, exp1)?;
                c.tex.push_str("}");
                c.last_control = false;
            }else{
                write_exp(c, exp1)?;
            }

            c.tex.push_str("_{");
            write_exp(c, exp2)?;
            c.tex.push_str("}");
            c.last_control = false;
        },

        Exp::ESuper(exp1, exp2) => {
            if is_fancy(exp1){
                write_grouped_exp(c, exp1)?;
                c.last_control = false;
            }else{
                write_exp(c, exp1)?;
            }

            c.tex.push_str("^");
            write_grouped_exp(c, exp2)?;
            c.last_control = false;
        },

        Exp::ESubsup(exp1, exp2, exp3) => {
            if is_fancy(exp1){
                write_grouped_exp(c, exp1)?;
                c.last_control = false;
            }else{
                write_exp(c, exp1)?;
            }

            c.tex.push_str("_");
            write_grouped_exp(c, exp2)?;
            c.tex.push_str("^");
            write_grouped_exp(c, exp3)?;
            c.last_control = false;
        },

        Exp::ESqrt(exp) => {
            c.last_control = false;
            c.tex.push_str("\\sqrt");
            write_grouped_exp(c, exp)?;
        },

        Exp::EFraction(fraction_type, exp1, exp2) => {
            c.last_control = false;
            c.tex.push_str("\\");
            c.tex.push_str(&fraction_type.to_str());
            write_grouped_exp(c, exp1)?;
            write_grouped_exp(c, exp2)?;
        },

        Exp::EText(text_type, str) => {
            c.last_control = false;
            if str.len() == 0{
                return Ok(());
            }
            let (cmd, repeats) = get_text_cmd(text_type);
            c.tex.push_str(&cmd);
            c.tex.push_str(&get_tex_math_many(str, &c.envs));
            c.tex.push_str("}".repeat(repeats as usize).as_str());
        },

        Exp::EStyled(text_type, exp_list) => {
            c.last_control = false;
            let cmd = get_style_latex_cmd(text_type, &c.envs);
            c.tex.push_str(cmd.as_str());
            c.tex.push_str("{");
            for exp in exp_list{
                write_exp(c, exp)?;
            }
            c.tex.push_str("}");
        },

        Exp::EPhantom(exp) => {
            c.last_control = false;
            c.tex.push_str("\\phantom{");
            write_exp(c, exp)?;
            c.tex.push_str("}");
        },

        Exp::EArray(alignments, exp_lists) => {
            // 根据alignments和amsmath环境来决定是使用array还是matrix还是aligned
            // matrix: amsmath环境下, aligns全部是AlignCenter
            // aligned: amsmath环境下, aligns是RL序列
            // array: 其他情况
            c.last_control = false;
            let null_aligns = &Vec::<Alignment>::new();
            let(name, aligns, rows) =
                match (aligns_is_rlsequence(alignments), aligns_is_all_center(alignments), c.envs["amsmath"]) {
                (true, false, true) => {
                    // self.write_array_table("aligned", &Vec::<Alignment>::new(), exp_lists);
                    // self.last_cmd = TexSeqType::Control;
                    // Ok(())
                    ("aligned", null_aligns, exp_lists)
                },
                (false, true, true) => {
                    // self.write_array_table("matrix", &Vec::<Alignment>::new(), exp_lists);
                    // self.last_cmd = TexSeqType::Control;
                    // Ok(())
                    ("matrix", null_aligns, exp_lists)
                },
                _ => {
                    // self.write_array_table("array", alignments, exp_lists);
                    // self.last_cmd = TexSeqType::Control;
                    // Ok(())
                    ("array", alignments, exp_lists)
                }
            };

            write_array_table(c, name, aligns, rows)?;
        },

        Exp::EOver(convertible, b, e1) => {
            c.last_control = false;
            match get_xarrow(b){
                Some(exp) => {
                    if c.envs["amsmath"]{
                        c.tex.push_str(exp.as_str());
                        c.tex.push_str("{");
                        write_exp(c, e1)?;
                        c.tex.push_str("}");
                    }
                },
                None => {
                    write_script(c, &Position::Over, convertible, b, e1)?;
                }
            };
        },

        Exp::EUnder(convertible, base, e1) => {
            c.last_control = false;
            write_script(c, &Position::Under, convertible, base, e1)?;
        },

        Exp::EUnderOver(convertible, b, e1, e2) => {
            c.last_control = false;
            // 特殊处理Accent重音符号
            if write_underover_accent(c, b){
                return Ok(());
            }

            match get_xarrow(b){
                Some(e) =>{
                    if c.envs["amsmath"]{
                        c.tex.push_str(e.as_str());
                        c.tex.push_str("[{");
                        write_exp(c, e1)?;
                        c.tex.push_str("}]{");
                        write_exp(c, e2)?;
                        c.tex.push_str("}");
                        return Ok(());
                    }
                }
                None => {
                    if is_operator(b){
                        if is_fancy(b){
                            write_grouped_exp(c, b)?;
                        }else{
                            // TODO: 可能要增加convertible对write_tex的影响
                            if *convertible{
                                write_exp(c, b)?;
                            }else{
                                c.tex.push_str("\\limits");
                            }
                            c.tex.push_str("_");
                            // TODO: check_substack
                            // check_substack(res, e1, envs);
                            write_grouped_exp(c, e1)?;
                            c.tex.push_str("^");
                            // check_substack(res, e2, envs);
                            write_grouped_exp(c, e2)?;
                            c.tex.push_str("");
                        }
                        return Ok(());
                    }
                }
            }
            // TODO: underover
            // writeExp (EUnder convertible (EOver convertible b e2) e1)
            panic!("writeExp (EUnder convertible (EOver convertible b e2) e1) not implemented");
        },

        Exp::ERoot(exp1, exp2) => {
            c.last_control = false;
            c.tex.push_str("\\sqrt[");
            write_exp(c, exp1)?;
            c.tex.push_str("]");
            write_exp(c, exp2)?;
        },

        Exp::EScaled(size, e) => {
            c.last_control = false;
            let flag = match **e {
                Exp::ESymbol(TeXSymbolType::Open, _) => true,
                Exp::ESymbol(TeXSymbolType::Close, _) => true,
                _ => false,
            };
            if flag{
                if let Some(cmd) = get_scaler_cmd(&size){
                    c.tex.push_str(cmd.as_str());
                }
                write_exp(c, e)?;
            }else{
                write_exp(c, e)?;
            }
        },
    }
    Ok(())
}
impl Alignment{
    fn to_str(&self) -> String{
        match self{
            Alignment::AlignLeft => {
                "l".to_string()
            },

            Alignment::AlignRight => {
                "r".to_string()
            },

            Alignment::AlignCenter => {
                "c".to_string()
            },
        }
    }
}

impl FractionType{
    fn to_str(&self) -> String{
        match self{
            FractionType::NormalFrac => {
                "frac".to_string()
            },

            FractionType::DisplayFrac => {
                "dfrac".to_string()
            },

            FractionType::InlineFrac => {
                "tfrac".to_string()
            },

            FractionType::NoLineFrac => {
                "binom".to_string()
            },
        }
    }
}


impl TeXSymbolType{
    fn to_show(&self) -> Result<String, String>{
        match self{
            TeXSymbolType::Bin => {
                Ok("bin".to_string())
            },
            TeXSymbolType::Rel => {
                Ok("rel".to_string())
            },
            TeXSymbolType::Open => {
                Ok("open".to_string())
            },
            _ => {
                Err(format!("TeXSymbolType {:?} not implemented", self))
            },
        }
    }
}