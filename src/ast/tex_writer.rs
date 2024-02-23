use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use crate::ast::to_tex_unicode::{escapse_text, get_math_tex_many};
use crate::pretty_print_hex;
use super::{judge, shared, to_tex_unicode};
use super::shared::is_mathoperator;
use super::node::{Alignment, ArrayLines, Exp, FractionType, InEDelimited, Rational, TeXSymbolType, TextType};

// Tex
// #[derive(Debug, PartfialEq)]
// enum Tex{
//     Control(String), // 控制序列: \frac, \alpha, \beta, \gamma
//     Token(char),
//     Literal(String),
//     Space,
//     Group(Vec<Tex>),
// }

pub struct TexWriterContext {
    tex: String, // 输出的文本
    envs: HashMap<String, bool>,
    need_space: bool, // 对于\alpha\beta这种, 如果遇到字母, 需要输出空格: \alphax\beta -> \alpha x\beta
    convertible: bool, // 是否可转换
}

impl TexWriterContext {
    pub fn default() -> Self {
        let mut envs = HashMap::new();
        envs.insert("amsmath".to_string(), true);
        envs.insert("amssymb".to_string(), true);
        envs.insert("mathbb".to_string(), true);
        TexWriterContext {
            tex: String::new(),
            envs,
            need_space: false,
            convertible: false,
        }
    }

    // 硬性添加空格, 会检查是否需要添加空格
    fn push_space(&mut self){
        // -- No space before ^, _, or \limits, and no doubled up spaces
        // ps = [ "^", "_", " ", "\\limits" ]

        if self.tex.ends_with(' ')
            || self.tex.ends_with('^')
            || self.tex.ends_with('_')
            || self.tex.ends_with("\\limits")
            || self.tex.ends_with("{") {
            // 如果最后一个字符是空格, 则不输出空格
            return;
        }

        self.tex.push(' ');
    }
    // 添加文本, 不会考虑是否需要添加空格
    fn push_raw(&mut self, s: &str){
        self.tex.push_str(s);
    }
    // 添加文本, 会考虑是否需要添加空格, 用于修正:
    // 1. \alphax\beta -> \alpha x\beta
    fn push_text(&mut self, s: &str) {
        if s.len() == 0 {
            return;
        } else if s == "}"{
            // }的前面如果有空格, 且不是\\ }, 则删除空格
            if self.tex.ends_with(' ') && !self.tex.ends_with("\\ "){
                self.tex.pop();
            }
        }else if self.need_space && s.chars().next().unwrap().is_ascii_alphanumeric(){
            // 上一个指示需要空格, 且当前是字母或数字, 则需要输出空格以分隔
            if !self.tex.ends_with(' '){
                self.tex.push(' ');
            }
        }

        self.tex.push_str(s);

        // \\开头且为字母结尾, 下一次调用的时候可能需要输出空格
        if s.starts_with("\\") && s.chars().rev().next().unwrap().is_ascii_alphabetic(){
            self.need_space = true;
        }else{
            // 其他情况下, 不需要输出空格
            self.need_space = false;
        }
    }
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
        native = file_content[start + "<<< native".len()..end].to_string().trim().to_string().replace("\r\n", "\n");
        o_tex = file_content[end + ">>> tex".len()..].to_string().trim().to_string().replace("\r\n", "\n");
    }

    let exp = super::ast_reader::read_ast(&native).unwrap();
    dbg!(exp.clone());
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    envs.insert("mathbb".to_string(), true);
    let tex = write_tex_with_env(exp, &envs).unwrap().trim().to_string();
    
    let f = fs::File::create("./output").unwrap();
    let mut f = std::io::BufWriter::new(f);
    // let judge_by_render = judge::judge_by_mathjax(native.clone(), tex.clone());
    // println!("judge_by_render: {:?}", judge_by_render);
    // if judge_by_render == false {
    //     return;
    // }
    let (jr, texmath) = judge::judge_by_texmath(o_tex.clone(), tex.clone());

    println!("A: {:?}", tex.as_bytes());
    println!("B: {:?}", o_tex.as_bytes());
    println!("C: {:?}", texmath.as_bytes());

    println!("same: {}", jr.to_str());

    f.write("same:".as_bytes()).unwrap();
    f.write(jr.to_str().to_string().as_bytes()).unwrap();
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

    // bytes hex:
    f.write(pretty_print_hex(o_tex.clone()).as_bytes()).unwrap();
    f.write("\n".as_bytes()).unwrap();

    f.write(pretty_print_hex(tex.clone()).as_bytes()).unwrap();
    f.write("\n".as_bytes()).unwrap();
}
// 把Exp转换为TeX, 带上环境
pub fn write_tex_with_env(exps: Vec<Exp>, envs: &HashMap<String, bool>) -> Result<String, String>{
    let twc = &mut TexWriterContext {
        tex: String::new(),
        need_space: false,
        envs: envs.clone(),
        convertible: false,
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
            c.push_text("{");
            for e in exp_list{
                write_exp(c, e)?;
            }
            c.push_text("}");
            Ok(())
        },
        _ => {
            c.push_text("{");
            write_exp(c, exp)?;
            c.push_text("}");
            Ok(())
        }
    }
}

// write_array_aligns:
// [AlignCenter, AlignCenter, AlignCenter] -> {ccc}
// 注意后面会有一个换行符
fn write_array_aligns(c: &mut TexWriterContext, aligns: &Vec<Alignment>) {
    // if has aligns
    if aligns.len() > 0 {
        c.push_text("{");
        c.push_text(get_alignments(aligns).as_str());
        c.push_text("}");
    }
    c.push_text("\n");
}
// write_array_rows:
// exp1 & exp2 & exp3 \\
// exp4 & exp5 & exp6
fn write_array_rows(c: &mut TexWriterContext, rows: &Vec<ArrayLines>) -> Result<(), String> {
    // array rows
    for (i, row) in rows.iter().enumerate(){
        for (j, ele) in row.iter().enumerate(){
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

            // 用 & 连接元素, 最后一个元素不需要输出&
            if j != row.len() - 1{
                // 如果前面的元素已经有空格, 则不输出空格
                if c.tex.chars().last().unwrap() != ' '{
                    c.tex.push(' ');
                }
                c.push_text("& ");
            }
        }

        if i != rows.len() - 1{
            // 如果不是最后一行, 需要输出空格+\\, 但是如果上一个元素已经有空格, 则不输出空格
            if c.tex.chars().last().unwrap() != ' '{
                c.tex.push(' ');
            }
            c.push_text("\\\\");
        }
        c.push_text("\n");
    }

    Ok(())
}
// 输出array table
// name = "array" or "matrix"...
fn write_array_table(c: &mut TexWriterContext, name: &str, aligns: &Vec<Alignment>, rows: &Vec<ArrayLines>) -> Result<(), String>{
    // \begin{array}{ccc}
    // 1 & 2 & 3 \\
    // 4 & 5 & 6
    // \end{array}
    c.push_text("\\begin{");
    c.push_text(name);
    c.push_text("}");

    write_array_aligns(c, aligns);
    write_array_rows(c, rows)?;

    c.push_text("\\end{");
    c.push_text(name);
    c.push_text("}");
    Ok(())
}

// 当Delimited只有一个Right元素且里面是EArray时调用
// Delimited open close [Right (EArray [AlignCenter] [[[x]],[[y]]])]
fn delimited_write_right_array(c: &mut TexWriterContext, open: &String, close: &String, aligns: &Vec<Alignment>, rows: &Vec<ArrayLines>) -> Result<(), String> {
    match (c.envs["amsmath"], open.as_str(), close.as_str()) {
        (true, "{", "") => {
            if aligns.len() == 2 && aligns[0] == Alignment::AlignLeft && aligns[1] == Alignment::AlignLeft {
                // \begin{cases} \end{cases}
                write_array_table(c, "cases", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        (true, "(", ")") => {
            if aligns_is_all_center(aligns) {
                // \begin{pmatrix} \end{pmatrix}
                write_array_table(c, "pmatrix", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        (true, "[", "]") => {
            if aligns_is_all_center(aligns) {
                // \begin{bmatrix} \end{bmatrix}
                write_array_table(c, "bmatrix", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        (true, "{", "}") => {
            if aligns_is_all_center(aligns) {
                // \begin{Bmatrix} \end{Bmatrix}
                write_array_table(c, "Bmatrix", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        (true, "\\8739", "\\8739") => {
            if aligns_is_all_center(aligns) {
                // \begin{vmatrix} \end{vmatrix}
                write_array_table(c, "vmatrix", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        (true, "\\8741", "\\8741") => {
            if aligns_is_all_center(aligns) {
                // \begin{Vmatrix} \end{Vmatrix}
                write_array_table(c, "Vmatrix", &Vec::<Alignment>::new(), rows)?;
                return Ok(());
            }
        }
        _ => {
            // other cases go below
        },
    };

    delimited_write_delim(c, FenceType::DLeft, open);
    write_exp(c, &Exp::EArray(aligns.clone(), rows.clone()))?;
    delimited_write_delim(c, FenceType::DRight, close);
    Ok(())
}

fn write_binom(c: &mut TexWriterContext, cmd: &str, e1: &Exp, e2: &Exp) -> Result<(), String>{
    if c.envs["amsmath"]{
        match cmd{
            "\\choose" => {
                c.push_text("\\binom");
            },
            "\\brack" => {
                c.push_text(get_general_frac("[", "]").as_str());
            },
            "\\brace" => {
                c.push_text(get_general_frac("\\{", "\\}").as_str());
            },
            "\\bangle" => {
                c.push_text(get_general_frac("\\langle", "\\rangle").as_str());
            },
            _ => {
                return Err(format!("unknown cmd in write_binom: {}", cmd));
            }
        }
        write_grouped_exp(c, e1)?;
        write_grouped_exp(c, e2)?;
    }else{
        // 不是这些, 则直接输出
        write_exp(c, e1)?;
        c.push_text(cmd);
        write_exp(c, e2)?;
    }
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
        // 左右尖括号
        ("\\10216", "\\10217") => {
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
    let tex_delim = escape_text_as_tex(delim, &c.envs);
    let valid = to_tex_unicode::is_delimiters(delim, &c.envs); // 界定符号是否有效

    let null_lim = escape_text_as_tex(".", &c.envs); // TODO: 空的界定符号

    let delim_cmd = match valid {
        true => tex_delim.clone(),
        false => null_lim,
    }; // 如果有效则使用tex_delim, 否则使用null_lim(空的界定符号)

    match ft {
        FenceType::DLeft => {
            // valid: \left(
            // invalid: \left. tex
            c.push_text("\\left");
            c.push_text(&delim_cmd);
            c.push_space();
            if !valid {
                c.push_text(&tex_delim);
            }
        },
        FenceType::DMiddle => {
            if valid{
                c.push_space();
                c.push_text("\\middle");
                c.push_text(&delim_cmd);
                c.push_space();
            }else{
                c.push_text(&tex_delim);
            }
        },
        FenceType::DRight => {
            c.push_space();
            c.push_text("\\right");
            c.push_text(&delim_cmd);
            if !valid {
                c.push_text(&tex_delim);
            }
        },
    }
}

#[test]
fn test_delimited_write_general_exp(){
    let mut c = TexWriterContext {
        tex: String::new(),
        need_space: false,
        envs: HashMap::new(),
        convertible: false,
    };
    // (EDelimited
    // "\10216"
    // "\10217"
    // [ Right (EIdentifier "H")
    //     , Right (ESymbol Rel "\8739")
    //     , Right (EIdentifier "H")
    // ])

    let open = "\\10216".to_string();
    let close = "\\10217".to_string();
    let exp_list = vec![
        InEDelimited::Right(Exp::EIdentifier("H".to_string())),
        InEDelimited::Right(Exp::ESymbol(TeXSymbolType::Rel, "\\8739".to_string())),
        InEDelimited::Right(Exp::EIdentifier("H".to_string())),
    ];
    delimited_write_general_exp(&mut c, &open, &close, &exp_list).unwrap();
    println!("res: {:?}", c.tex);
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
            ("|", "|") => {
                true
            },
            _ => {
                false
            }
        };

    let is_right = is_all_right(exp_list);
    let is_standard_height = is_all_standard_height(exp_list);
    return if is_open_close && is_right && is_standard_height {
        c.push_text(&escape_text_as_tex(open, &c.envs));
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
        c.push_text(&escape_text_as_tex(close, &c.envs));
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

#[test]
fn test_write_script(){
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    let mut c = TexWriterContext {
        tex: String::new(),
        need_space: false,
        envs,
        convertible: false,
    };
    // EUnder False (ESymbol Op "\8749") (EIdentifier "S")
    let b = Exp::ESymbol(TeXSymbolType::Op, "\\8749".to_string());
    let e1 = Exp::EIdentifier("S".to_string());
    write_script(&mut c, &Position::Under, &false, &b, &e1).unwrap();
    println!("res: {:?}", c.tex);
}
fn write_script(c: &mut TexWriterContext, p: &Position, convertible: &bool, b: &Exp, e1: &Exp) -> Result<(), String>{
    let dia_cmd = match e1{
        Exp::ESymbol(t, s) => {
            if t == &TeXSymbolType::Accent || t == &TeXSymbolType::TOver || t == &TeXSymbolType::TUnder {
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
        c.push_text(&cmd);
        write_grouped_exp(c, b)?;
        return Ok(());
    }

    if is_operator(b){
        if *convertible{
            c.convertible = true;
        }

        if is_fancy(b){
            write_grouped_exp(c, b)?;
        }else{
            write_exp(c, b)?;
        }

        if !*convertible{
            c.push_text("\\limits");
        }

        match p{
            Position::Under => {
                c.push_text("_");
            },
            Position::Over => {
                c.push_text("^");
            }
        }

        write_if_substack(c, e1)?;
        c.convertible = false; // reset
        return Ok(());
    }else if p==&Position::Over && e1 == &Exp::ESymbol(TeXSymbolType::Accent, "\\831".to_string()){
        // 特殊情况的处理: \831 -> \u{033F}, unicode中表示上双横线 -> 用\overline{\overline{b}}代替
        // double bar
        // tell [ControlSeq "\\overline", Literal "{",
        // ControlSeq "\\overline"]
        // tellGroup (writeExp b)
        // tell [Literal "}"]

        c.push_text("\\overline{\\overline");
        write_grouped_exp(c, b)?;
        c.push_text("}");
    }else{
        // case pos of
        // Over   -> tell [ControlSeq "\\overset"]
        // Under  -> tell [ControlSeq "\\underset"]
        // tellGroup (writeExp e1)
        // tellGroup (writeExp b)
        match p {
            Position::Over => {
                c.push_text("\\overset");
            },
            Position::Under => {
                c.push_text("\\underset");
            }
        }

        write_grouped_exp(c, e1)?;
        write_grouped_exp(c, b)?;
    }

    Ok(())
}

// 在underover中其中一个是accent时调用
fn write_underover_accent(c: &mut TexWriterContext, exp: &Exp) -> Result<bool, String>{
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
                write_exp(c, &new_under)?;
                return Ok(true);
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
                write_exp(c, &new_over)?;
                return Ok(true);
            }
            Ok(false)
        },
        _ => {
            Ok(false)
        }
    }
}

// 在某个字符下面书写多行文本时调用, 如\sum:
// \sum_{\substack{0 \le i \le m \\ 0 \le j \le n}} a_{i,j}
// 如果不符合条件, 则调用writeExp
fn write_if_substack(c: &mut TexWriterContext, e:&Exp) -> Result<(), String>{
    // (EArray [AlignCenter] rows) 模式且 envs["amsmath"] = True
    // Otherwise -> writeExp e
    if let Exp::EArray(aligns, rows) = e {
        if c.envs["amsmath"] && aligns.len() == 1 && aligns[0] == Alignment::AlignCenter {
            c.push_text("\\substack{");
            write_array_rows(c, rows)?;
            c.push_text("}");
            return Ok(());
        }
    }

    return write_exp(c, e);
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
fn escape_text_as_tex(s: &str, envs: &HashMap<String, bool>) -> String{
    let (res, _) = get_math_tex_many(s, envs);
    return res
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



fn get_scaler_cmd(rational: &Rational) -> Option<String>{
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

fn get_diacritical_cmd(pos: &Position, s: &str) -> Option<String>{
    let cmd = shared::get_diacriticals(s);

    match cmd {
        Some(cmd) => {
            if cmd == "\\overbracket" || cmd == "\\underbracket" {
                // -- We want to parse these but we can't represent them in LaTeX
                // unavailable :: [T.Text]
                // unavailable = ["\\overbracket", "\\underbracket"]
                return None;
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
        need_space: false,
        convertible: false,
    };
    for exp in &exps {
        write_exp(tw, exp)?;
    }
    Ok(())
}


fn write_exp(c: &mut TexWriterContext, exp: &Exp) -> Result<(), String>{
    match exp{
        Exp::ENumber(n) => {
            c.push_text(escape_text_as_tex(n, &c.envs).as_str());
        },

        Exp::EBoxed(exp) => {
            if c.envs["amsmath"]{
                c.push_text("\\boxed");
                write_grouped_exp(c, exp)?;
            }else{
                write_exp(c, exp)?;
            }
        },

        Exp::EGrouped(exp_list) => {
            // 如果只有一个元素, 则不需要{}
            if exp_list.len() == 1{
                write_exp(c, &exp_list[0])?;
            }else{
                c.push_text("{");
                for exp in exp_list{
                    write_exp(c, exp)?;
                }
                c.push_text("}");
            }

        },

        Exp::EDelimited(left, right, exp_list) => {
            if exp_list.len() == 1{
                match &exp_list[0] {
                    // EDelimited open close [Right (EFraction NoLineFrac e1 e2)]
                    InEDelimited::Right(Exp::EFraction(FractionType::NoLineFrac, e1, e2)) => {
                        return delimited_fraction_noline(c, left, right, e1, e2);
                    },
                    // EDelimited open close [Right (EArray aligns rows)]
                    InEDelimited::Right(Exp::EArray(aligns, rows)) => {
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
            let escaped = escape_text_as_tex(&symbol, &c.envs);
            // 如果是Bin, Rel则需要添加一个空格
            if *symbol_type == TeXSymbolType::Bin || *symbol_type == TeXSymbolType::Rel{
                c.push_space();
            }

            if symbol.chars().next().unwrap() != '\\' && symbol.len() > 1 &&
                (symbol_type == &TeXSymbolType::Bin || symbol_type == &TeXSymbolType::Rel || symbol_type == &TeXSymbolType::Op) {
                // 多字符的非控制关系符, 如要显示: a < b or a > b
                // 这种情况下直接渲染的话, bora会黏在一起, 需要指示 \mathrel{\text{or}}

                let content = match symbol_type {
                    TeXSymbolType::Bin => {
                        "bin"
                    },
                    TeXSymbolType::Rel => {
                        "rel"
                    },
                    TeXSymbolType::Op => {
                        "op"
                    },
                    _ => panic!("error in write_exp ESymbol: symbol_type is not Bin, Rel or Op"),
                };
                c.push_text(&("\\math".to_owned() + content + "{\\text{"));
                c.push_text(&escaped);
                c.push_text("}}");
            }else{
                c.push_text(&escaped);
            }

            // 如果是Bin, Rel则需要添加一个空格
            if *symbol_type == TeXSymbolType::Bin || *symbol_type == TeXSymbolType::Rel{
                c.push_space();
            }
        },

        // ok
        Exp::ESpace(rational) => {
            let width = rational.numerator as f32 / rational.denominator as f32 * 18.0;
            let width = width.floor() as i32;
            match width {
                -3 => {
                    c.push_text("\\!");
                },
                0 => {},
                3 => {
                    c.push_text("\\,");
                },
                4 => {
                    // use: \\  \\: \\>
                    c.push_text("\\ ");
                },
                5 => {
                    c.push_text("\\;");
                },
                18 => {
                    c.push_text("\\quad");
                    return Ok(());
                },
                36 => {
                    c.push_text("\\qquad");
                    return Ok(());
                },
                n => {
                    if c.envs["amsmath"]{
                        c.push_text("\\mspace{");
                        c.push_text(&n.to_string());
                        c.push_text("mu}");
                    }else{
                        c.push_text("\\mskip{");
                        c.push_text(&n.to_string());
                        c.push_text("mu}");
                    }
                }
            }
        },

        Exp::EIdentifier(identifier) => {
            // 为了防止连续的标识符被合并, 需要在标识符之间添加空格, 如:
            // \alphax -> \alpha x
            let (escaped, nums) = get_math_tex_many(&identifier, &c.envs);
            if escaped.len() == 0{
                return Ok(());
            }

            if nums > 1{
                // 检查外层有没有括号, 如果有则不需要添加{}
                // TODO: 检测不完全, 外面的括号可能是其他的
                if c.tex.len() > 0 && c.tex.chars().last().unwrap() == '{'{
                    c.push_text(escaped.as_str());
                }else{
                    c.push_text("{");
                    c.push_text(escaped.as_str());
                    c.push_text("}");
                }
            }else{
                c.push_text(&escaped);
            }

        },

        Exp::EMathOperator(math_operator) => {
            let escaped = escape_text_as_tex(&math_operator, &c.envs);

            if is_mathoperator(escaped.as_str()) {
                c.push_text(format!("\\{}", escaped).as_str());
            }else{
                if c.convertible{
                    c.push_text("\\operatorname*{");
                }else{
                    c.push_text("\\operatorname{");
                }
                c.push_text(&escaped);
                c.push_text("}");
            }
        },

        Exp::ESub(exp1, exp2) => {
            if is_fancy(exp1){
                write_grouped_exp(c, exp1)?;
            }else{
                write_exp(c, exp1)?;
            }

            c.push_text("_");
            write_grouped_exp(c, exp2)?;
        },

        Exp::ESuper(exp1, exp2) => {
            if is_fancy(exp1){
                write_grouped_exp(c, exp1)?;
            }else{
                write_exp(c, exp1)?;
            }

            c.push_text("^");
            write_grouped_exp(c, exp2)?;
        },

        Exp::ESubsup(exp1, exp2, exp3) => {
            if is_fancy(exp1){
                write_grouped_exp(c, exp1)?;
            }else{
                write_exp(c, exp1)?;
            }

            c.push_text("_");
            write_grouped_exp(c, exp2)?;
            c.push_text("^");
            write_grouped_exp(c, exp3)?;
        },

        Exp::ESqrt(exp) => {
            c.push_text("\\sqrt");
            write_grouped_exp(c, exp)?;
        },

        Exp::EFraction(fraction_type, exp1, exp2) => {
            c.push_text(format!("\\{}", fraction_type.to_str()).as_str());
            write_grouped_exp(c, exp1)?;
            write_grouped_exp(c, exp2)?;
        },

        Exp::EText(text_type, str) => {
            if str.len() == 0{
                return Ok(());
            }
            let (cmd, repeats) = get_text_cmd(text_type);
            let text = &escapse_text(str);

            c.push_text(&format!("{}{}{}", cmd, text, "}".repeat(repeats as usize)));
        },

        Exp::EStyled(text_type, exp_list) => {
            let cmd = get_style_latex_cmd(text_type, &c.envs);
            c.push_text(cmd.as_str());
            c.push_text("{");
            for exp in exp_list{
                write_exp(c, exp)?;
            }
            c.push_text("}");
        },

        Exp::EPhantom(exp) => {
            c.push_text("\\phantom");
            write_grouped_exp(c, exp)?;
        },

        Exp::EArray(alignments, exp_lists) => {
            // 根据alignments和amsmath环境来决定是使用array还是matrix还是aligned
            // matrix: amsmath环境下, aligns全部是AlignCenter
            // aligned: amsmath环境下, aligns是RL序列
            // array: 其他情况
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
            if let Some(exp) = get_xarrow(b){
                if c.envs["amsmath"]{
                    c.push_text(exp.as_str());
                    write_grouped_exp(c, e1)?;
                    return Ok(());
                }
            }

            write_script(c, &Position::Over, convertible, b, e1)?;
        },

        Exp::EUnder(convertible, base, e1) => {
            write_script(c, &Position::Under, convertible, base, e1)?;
        },

        Exp::EUnderOver(convertible, b, e1, e2) => {

            // (EUnderover convertible b e1@(ESymbol Accent _) e2)
            // (EUnderover convertible b e1 e2@(ESymbol Accent _))

            // 特殊处理Accent重音符号

            match write_underover_accent(c, b) {
                Ok(true) => {
                    return Ok(());
                },
                Err(e) => {
                    return Err(e);
                },
                _ => {
                    // go to below
                }
            };

            // xarrow: 在amsmath环境下, \xrightarrow, \xleftarrow
            // 在箭头上下加上文本
            // \xrightarrow[below]{above}
            if let Some(exp) = get_xarrow(b){
                if c.envs["amsmath"]{
                    // \xrightarrow[below]{above}
                    c.push_text(exp.as_str());
                    c.push_text("[");
                    write_grouped_exp(c, e1)?;
                    c.push_text("]");
                    write_grouped_exp(c, e2)?;
                    return Ok(());
                }
            }

            if is_operator(b){
                if *convertible{
                    c.convertible = true;
                }

                if is_fancy(b){
                    write_grouped_exp(c, b)?;
                }else{
                    write_exp(c, b)?;
                }

                if !*convertible{
                    c.push_text("\\limits");
                }
                c.push_text("_");
                write_if_substack(c, e1)?;
                c.push_text("^");
                write_if_substack(c, e2)?;

                c.convertible = false; // reset

                return Ok(());
            }
            // writeExp (EUnder convertible (EOver convertible b e2) e1)
            write_exp(c, &Exp::EUnder(
                *convertible,
                Box::new(Exp::EOver(
                    *convertible,
                    (*b).clone(),
                    (*e2).clone()
                )),
                (*e1).clone()
            ))?;
        },

        Exp::ERoot(exp1, exp2) => {
            c.push_text("\\sqrt[");
            write_exp(c, exp1)?;
            c.push_text("]");
            write_grouped_exp(c, exp2)?;
        },

        Exp::EScaled(size, e) => {
            let flag = match **e {
                Exp::ESymbol(TeXSymbolType::Open, _) => true,
                Exp::ESymbol(TeXSymbolType::Close, _) => true,
                _ => false,
            };
            if flag{
                if let Some(cmd) = get_scaler_cmd(&size){
                    c.push_text(cmd.as_str());
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
