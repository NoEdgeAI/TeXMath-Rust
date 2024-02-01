use core::panic;
use std::collections::HashMap;
use std::f32::consts::E;
use std::fmt::write;
use std::option;

use super::node;
use super::to_tex_unicode;
use super::node::ArrayLines;
use super::node::Exp;

pub struct TexWriter{
    e : Vec<node::Exp>,
    envs : HashMap<String, bool>,
}

#[test]
fn test_tex_writer(){
    let case = r#"
    [ EArray
    [ AlignLeft , AlignLeft ]
    [ [ [ EText TextMonospace "textrm" ]
      , [ EText TextNormal "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mathrm" ]
      , [ EStyled
            TextNormal
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathup" ]
      , [ EStyled
            TextNormal
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "text" ]
      , [ EText TextNormal "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mbox" ]
      , [ EText TextNormal "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mathbf" ]
      , [ EStyled
            TextBold
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfup" ]
      , [ EStyled
            TextBold
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "textbf" ]
      , [ EText TextBold "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mathit" ]
      , [ EStyled
            TextItalic
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "textit" ]
      , [ EText TextItalic "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mathtt" ]
      , [ EStyled
            TextMonospace
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "texttt" ]
      , [ EText TextMonospace "ABCabc" ]
      ]
    , [ [ EText TextMonospace "mathsf" ]
      , [ EStyled
            TextSansSerif
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathsfup" ]
      , [ EStyled
            TextSansSerif
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbb" ]
      , [ EStyled
            TextDoubleStruck
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathcal" ]
      , [ EStyled
            TextScript
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathscr" ]
      , [ EStyled
            TextScript
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathfrak" ]
      , [ EStyled
            TextFraktur
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfit" ]
      , [ EStyled
            TextBoldItalic
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfsfup" ]
      , [ EStyled
            TextSansSerifBold
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfsfit" ]
      , [ EStyled
            TextSansSerifBoldItalic
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfscr" ]
      , [ EStyled
            TextBoldScript
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbffrak" ]
      , [ EStyled
            TextBoldFraktur
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathbfcal" ]
      , [ EStyled
            TextBoldScript
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    , [ [ EText TextMonospace "mathsfit" ]
      , [ EStyled
            TextSansSerifItalic
            [ EIdentifier "A"
            , EIdentifier "B"
            , EIdentifier "C"
            , EIdentifier "a"
            , EIdentifier "b"
            , EIdentifier "c"
            ]
        ]
      ]
    ]
]"#;
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let exp = super::ast_reader::read_ast(case).unwrap();
    // dbg!(&exp);
    let tex = write_tex_with_env(exp, envs);
    println!("{}", tex);
}

// 把Exp转换为TeX, 带上环境
pub fn write_tex_with_env(exps: Vec<Exp>, envs: HashMap<String, bool>) -> String{
    let mut s = String::new();
    for exp in exps{
        write_tex(&exp,&mut s, &envs);
    }
    s
}

// 通用分数
fn write_gen_frac(s: &mut String, open: &str, close: &str){
    // \genfrac{left-delim}{right-delim}{thickness}{style}{numerator}{denominator}
    // \genfrac{左分隔符}{右分隔符}{厚度}{样式}{分子}{分母}
    // eg: \genfrac{[}{]}{0pt}{}{x}{y}
    s.push_str("\\genfrac{");
    s.push_str(open);
    s.push_str("}{");
    s.push_str(close);
    s.push_str("}{");
    s.push_str("0pt"); // 表示分数线的厚度, 0pt表示= 没有分数线
    s.push_str("}{");
    s.push_str("}");
}

// check if all exp is right
fn is_all_right(exp_list: &Vec<node::InEDelimited>) -> bool{
    for exp in exp_list{
        match exp {
            node::InEDelimited::Left(..) => {
                return false;
            },
            node::InEDelimited::Right(_) => {}
        }
    }
    return true;
}

// 把字符串的每一个字符转换为unicode escape
// 需要同时处理转义字符和utf8码点\d{4}
fn get_tex_math_many(s: &str, envs: &HashMap<String, bool>) -> String{
    // TODO: escape each char
    let mut res = String::new();
    
    to_tex_unicode::escape_single_symbol_unicode(s, envs)
}

#[test]
fn test_remove_outer_group(){
    let test_case = Exp::EGrouped(vec![
            Exp::ENumber("5".to_string()),
        ]
    );

    let res = remove_outer_group(&test_case);
    assert_eq!(res, &Exp::ENumber("5".to_string()));

    let test_case = Exp::EGrouped(vec![
            Exp::ENumber("5".to_string()),
            Exp::ENumber("6".to_string()),
        ]
    );

    let res = remove_outer_group(&test_case);
    assert_eq!(res, &test_case);
}

// remove outer group
// 如果Exp是EGrouped，且只有一个元素，则返回该元素
// 否则返回原来的Exp
fn remove_outer_group(exp: &Exp) -> &Exp{
    match exp{
        Exp::EGrouped(exp_list) => {
            if exp_list.len() == 1{
                return remove_outer_group(&exp_list[0]);
            }
            return exp;
        },
        _ => {
            return exp;
        }
    }
}

#[test]
fn test_is_all_standard_height(){
    let test_case = vec![
        node::InEDelimited::Right(Exp::ENumber("5".to_string())),
        node::InEDelimited::Right(Exp::ESymbol(node::TeXSymbolType::Bin, "\u{2212}".to_string())),
        node::InEDelimited::Right(Exp::EIdentifier("T".to_string())),
        ];

    assert_eq!(is_all_standard_height(&test_case), true);

    let test_case = vec![
        node::InEDelimited::Right(Exp::ENumber("5".to_string())),
        node::InEDelimited::Right(Exp::ESymbol(node::TeXSymbolType::Bin, "\u{2212}".to_string())),
        node::InEDelimited::Right(Exp::EIdentifier("T".to_string())),
        node::InEDelimited::Right(Exp::EGrouped(vec![
            Exp::ENumber("5".to_string()),
            Exp::ENumber("6".to_string()),
        ])),
        node::InEDelimited::Left(")".to_string()),
        ];
    
    assert_eq!(is_all_standard_height(&test_case), false);
}

// check if all exp is standard height:
// Right(ENumber, EIdentifier, ESpace, ESymbol(Ord, Op, Bin, Rel, Pun))
fn is_all_standard_height(exp: &Vec<node::InEDelimited>) -> bool{
    for e in exp{
        match e {
            node::InEDelimited::Left(..) => {
                return false;
            },
            node::InEDelimited::Right(exp) => {
                match exp{
                    Exp::ENumber(..) => {},
                    Exp::EIdentifier(..) => {},
                    Exp::ESpace(..) => {},
                    Exp::ESymbol(node::TeXSymbolType::Ord, ..) => {},
                    Exp::ESymbol(node::TeXSymbolType::Op, ..) => {},
                    Exp::ESymbol(node::TeXSymbolType::Bin, ..) => {},
                    Exp::ESymbol(node::TeXSymbolType::Rel, ..) => {},
                    Exp::ESymbol(node::TeXSymbolType::Pun, ..) => {},
                    _ => {
                        return false;
                    }
                }
            },
        }
    }
    return true;
}

fn write_binom(s: &mut String, control: &str, exp1: &Exp, exp2: &Exp, envs: &HashMap<String, bool>) {
    if envs["amsmath"]{
        match control {
            "\\choose" => {
                s.push_str("\\binom");
            },
            "\\brack" => {
                write_gen_frac(s, "[", "]");
            },
            "\\brace" => {
                write_gen_frac(s, "{", "}");
            },
            "\\bangle" => {
                write_gen_frac(s, "\\langle", "\\rangle");
            },
            _ => {
                panic!("writeBinom: unknown cmd");
            }
        };
        s.push_str("{");
        write_tex(exp1,s, envs);
        s.push_str("}{");
        write_tex(exp2,s, envs);
        s.push_str("}");
    }
}

// 输出alignments, 不带{}
// AlignLeft -> l, AlignRight -> r, AlignCenter -> c
fn write_alignments(s: &mut String, aligns: &Vec<node::Alignment>){
    for align in aligns{
        s.push_str(&align.to_str());
    }
}

#[test]
fn test_write_arraylines(){
    let case = vec![
        vec![
            vec![Exp::ENumber("1".to_string())],
            vec![Exp::ENumber("2".to_string())],
        ],
        vec![
            vec![Exp::ENumber("3".to_string())],
            vec![Exp::ENumber("4".to_string())],
        ],
    ];

    let mut s = String::new();
    write_arraylines(&mut s, &case, &HashMap::new());
    assert_eq!(s, "1 & 2 \\\\\n3 & 4");
}

// 输出array的单个元素
fn write_arrayline(s: &mut String, row: &Vec<node::Exp>, envs: &HashMap<String, bool>){
    if row.len() == 0{
        return;
    }else if row.len() == 1{
        write_tex(&row[0],s, envs);
        return;
    }else{
        panic!("writeArrayLine: multi elements not implemented")
    }
}

// 输出array rows:
// xxx & xxx & xxx \\
fn write_arraylines(s: &mut String, rows: &Vec<ArrayLines>, envs: &HashMap<String, bool>) {
    // doRows :: [ArrayLine] -> Math ()
    // doRows []          = return ()
    // doRows ([]:[])     = tell [Token '\n']
    // doRows ([]:ls)     = tell [Space, Literal "\\\\", Token '\n'] >> doRows ls
    // doRows ([c]:ls)    = cell c >> doRows ([]:ls)
    // doRows ((c:cs):ls) = cell c >> tell [Space, Token '&', Space] >> doRows (cs:ls)

    if rows.len() == 0{
        return;
    }else {
        for row in rows{
            for i in 0..row.len(){
                write_arrayline(s, &row[i], envs);
                if i != row.len() - 1{
                    s.push_str(" & ");
                }
            }

            if row == &rows[rows.len() - 1]{
                s.push_str("\n");
                continue; // 最后一行不需要输出\\
            }
            s.push_str(" ");
            s.push_str("\\\\");
            s.push_str("\n");
        }
    }
}

// 判断是否是RL序列: 
// RL序列是指以AlignRight开头，以AlignLeft结尾，中间可以有任意多个AlignRight和AlignLeft
fn aligns_is_rlsequence(aligns: &Vec<node::Alignment>) -> bool{
    // isRLSequence :: [Alignment] -> Bool
    // isRLSequence [AlignRight, AlignLeft] = True
    // isRLSequence (AlignRight : AlignLeft : as) = isRLSequence as
    // isRLSequence _ = False
    if aligns.len() % 2 == 0{
        for align_pair in aligns.chunks(2){
            if align_pair[0] != node::Alignment::AlignRight || align_pair[1] != node::Alignment::AlignLeft{
                return false;
            }
        }
        return true;
    }else{
        return false;
    }
}

// 判断是否是全部是AlignCenter, 这样的话可以使用matrix
fn aligns_is_all_center(aligns: &Vec<node::Alignment>) -> bool{
    for align in aligns{
        if align != &node::Alignment::AlignCenter{
            return false;
        }
    }
    return true;
}

// 输出array table
// name = "array" or "matrix"...
fn write_array_table(s: &mut String, name: &str, aligns: &Vec<node::Alignment>, rows: &Vec<ArrayLines>, envs: &HashMap<String, bool>){
    // \begin{xxx}
    // \begin{array}{ccc}

    s.push_str("\\begin{");
    s.push_str(name);
    s.push_str("}");
    
    // if has aligns
    if aligns.len() > 0 { 
        s.push_str("{");
        write_alignments(s, aligns);
        s.push_str("}");
    }

    s.push_str("\n");

    write_arraylines(s, rows, envs);

    s.push_str("\\end{");
    s.push_str(name);
    s.push_str("}");
}

// 当Delimited只有一个Right元素且里面是EArray时调用
// Delimited open close [Right (EArray [AlignCenter] [[[x]],[[y]]])]
fn delimited_write_right_array(s: &mut String, open: &String, close: &String, exp: &Vec<node::InEDelimited>, envs: &HashMap::<String, bool>) -> bool {
    if exp.len() != 1{
        return false;
    }
    let exp = &exp[0];
    match exp{
        node::InEDelimited::Left(..) => {
            return false;
        },
        node::InEDelimited::Right(exp) => {
            if let Exp::EArray(aligns, rows) = exp{
                if envs["amsmath"]{
                    match (open.as_str(), close.as_str()) {
                        ("{", "") => {
                            if aligns.len() == 2 && aligns[0] == node::Alignment::AlignLeft && aligns[1] == node::Alignment::AlignLeft{
                                // \begin{cases} \end{cases}
                                write_array_table(s, "cases", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        ("(",")") => {
                            if aligns_is_all_center(aligns){
                                // \begin{pmatrix} \end{pmatrix}
                                write_array_table(s, "pmatrix", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        ("[","]") => {
                            if aligns_is_all_center(aligns){
                                // \begin{bmatrix} \end{bmatrix}
                                write_array_table(s, "bmatrix", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        ("{","}") => {
                            if aligns_is_all_center(aligns){
                                // \begin{Bmatrix} \end{Bmatrix}
                                write_array_table(s, "Bmatrix", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        ("\u{2223}", "\u{2223}") => {
                            if aligns_is_all_center(aligns){
                                // \begin{vmatrix} \end{vmatrix}
                                write_array_table(s, "vmatrix", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        ("\u{2225}", "\u{2225}") => {
                            if aligns_is_all_center(aligns){
                                // \begin{Vmatrix} \end{Vmatrix}
                                write_array_table(s, "Vmatrix", &Vec::<node::Alignment>::new(), rows, envs);
                                return true;
                            }
                        }
                        _ => {},
                    }
                }
                // 以上都不是，那么就是一个普通的array
                delimited_write_delim(s, FenceType::DLeft, &open, envs);
                write_tex(exp,s, envs); // TODO: write array is ?
                delimited_write_delim(s, FenceType::DRight, &close, envs);
            }
        }
    }
    return false;
}

fn delimited_fraction_noline(s: &mut String, left: &String, right: &String, exp_list: &Vec<node::InEDelimited>, envs: &HashMap::<String, bool>) -> bool { 
    if exp_list.len() != 1{
        return false;
    }
    let exp = &exp_list[0];
    match exp {
        node::InEDelimited::Left(_) => {
            panic!("delimited in middle not implemented");
        },
        node::InEDelimited::Right(exp) => {
            if let Exp::EFraction(node::FractionType::NoLineFrac, e1, e2) = exp{
                let frac_exp1 = &e1;
                let frac_exp2: &&Box<Exp> = &e2;
                match (left.as_str(), right.as_str()){
                    ("(", ")") => {
                        // \choose
                        // return (true, write_binom("\\choose", frac_exp1, frac_exp2, envs));
                        write_binom(s, "\\choose", frac_exp1, frac_exp2, envs);
                    },
                    ("[", "]") => {
                        // \\brack
                        write_binom(s, "\\brack", frac_exp1, frac_exp2, envs);
                    },
                    ("{", "}") => {
                        // \\brace
                        write_binom(s, "\\brace", frac_exp1, frac_exp2, envs);
                    },
                    ("\u{27E8}", "\u{27E9}") =>{
                        // \\bangle
                        write_binom(s, "\\bangle", frac_exp1, frac_exp2, envs);
                    },
                    _ => {
                        // others:
                        // writeExp (EDelimited open close [Right (EArray [AlignCenter]
                        //     [[[x]],[[y]]])])
                        // TODO: delimited write right array
                        panic!("delimited_fraction_noline not implemented");
                    }
                }
            }
        }
    }
    return false;
}
enum FenceType{
    DLeft,
    DMiddle,
    DRight,
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

fn delimited_write_delim(s: &mut String, ft: FenceType, delim: &str, envs: &HashMap::<String, bool>){
    let tex_delim = get_tex_math_many(s, envs);
    let valid = is_delimiters(s, envs); // 界定符号是否有效
    let null_lim = get_tex_math_many(".", envs); // TODO: 空的界定符号

    let delim_cmd = match valid {
        true => tex_delim.clone(),
        false => null_lim,
    }; // 如果有效则使用tex_delim, 否则使用null_lim(空的界定符号)

    match ft {
        FenceType::DLeft => {
            // valid: \left( 
            // invalid: \left. tex
            s.push_str("\\left");
            s.push_str(&delim_cmd);
            s.push_str(" ");
            if !valid {
                s.push_str(&tex_delim);
            }
        },
        FenceType::DMiddle => {
            if valid{
                s.push_str(" ");
                s.push_str("\\middle");
                s.push_str(&delim_cmd);
                s.push_str(" ");
            }else{
                s.push_str(&tex_delim);
            }
        },
        FenceType::DRight => {
            s.push_str(" ");
            s.push_str("\\right");
            s.push_str(&delim_cmd);
            if !valid {
                s.push_str(&tex_delim);
            }
        },
    }
}
fn delimited_write_general_exp(s: &mut String, open: &String, close: &String, exp_list: &Vec<node::InEDelimited>, envs: &HashMap::<String, bool>){
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
//   isStandardHeight (Right (ESymbol ty _)) = ty `elem` [Ord, Op, Bin, Rel, Pun]
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
    if is_open_close && is_right && is_standard_height{
        s.push_str(&get_tex_math_many(open, envs));
        // mapM_ (either (writeDelim DMiddle) writeExp) es
        for exp in exp_list{
            match exp {
                node::InEDelimited::Left(delim) => {
                    delimited_write_delim(s, FenceType::DMiddle, delim, envs);
                },
                node::InEDelimited::Right(exp) => {
                    write_tex(exp,s, envs);
                }      
            }
        }
        s.push_str(&get_tex_math_many(close, envs));
        return;
    }else{
        // writeExp (EDelimited open close es) =  do
        // writeDelim DLeft open
        // mapM_ (either (writeDelim DMiddle) writeExp) es
        // writeDelim DRight close
        delimited_write_delim(s, FenceType::DLeft, open, envs);
        for exp in exp_list{
            match exp {
                node::InEDelimited::Left(delim) => {
                    delimited_write_delim(s, FenceType::DMiddle, delim, envs);
                },
                node::InEDelimited::Right(exp) => {
                    write_tex(exp,s, envs);
                }      
            }
        }
        delimited_write_delim(s, FenceType::DRight, close, envs);
        return;
    }    
}


fn get_scaler_cmd(rational: &node::Rational) -> Option<String>{
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
#[derive(PartialEq, Debug)]
enum Position{
    Under,
    Over,
}
fn write_script(s: &mut String, p: &Position, convertible: &bool, base: &node::Exp, e1: &node::Exp){
    // TODO: write script
}

fn write_underover_accent(s: &mut String, exp: &node::Exp, envs: &HashMap<String, bool>) -> bool{
    // writeExp (EUnder convertible (EOver False b e2) e1)
    // writeExp (EOver convertible (EUnder False b e1) e2)
    //           p1     convertible  p2     base  inner  outer
    
    // TODO: write underover accent
    return false;
}

fn check_substack(s: &mut String, e:&Exp, envs: &HashMap<String, bool>){
    // TODO: check substack
}

fn get_style_latex_cmd(style: &node::TextType, envs: &HashMap<String, bool>) -> String{
    // TODO: 处理环境, 有些环境可能不支持某些style, 如mathbfit
    match style{
        &node::TextType::TextNormal => "\\mathrm".to_string(),
        &node::TextType::TextBold => "\\mathbf".to_string(),
        &node::TextType::TextItalic => "\\mathit".to_string(),
        &node::TextType::TextMonospace => "\\mathtt".to_string(),
        &node::TextType::TextBoldItalic => "\\mathbfit".to_string(), 
        &node::TextType::TextSansSerif => "\\mathsf".to_string(),
        &node::TextType::TextSansSerifBold => "\\mathbfsf".to_string(),
        &node::TextType::TextSansSerifItalic => "\\mathbfsf".to_string(),
        &node::TextType::TextSansSerifBoldItalic => "\\mathbfsfit".to_string(),
        &node::TextType::TextScript => "\\mathcal".to_string(),
        &node::TextType::TextFraktur => "\\mathfrak".to_string(),
        &node::TextType::TextDoubleStruck => "\\mathbb".to_string(),
        &node::TextType::TextBoldFraktur => "\\mathbffrak".to_string(),
        &node::TextType::TextBoldScript => "\\mathbfscr".to_string(),
        _ => panic!("get_style_latex_cmd not implemented: {:?}", style),
    }
}

// 获取\text的cmd, 有可能有多个cmd
// 第二个返回值是cmd的个数, 添加{}的个数
fn get_text_cmd(t: &node::TextType) -> (String, u8){
    match t{
        &node::TextType::TextNormal => ("\\text{".to_string(),1),
        &node::TextType::TextBold => ("\\textbf{".to_string(),1),
        &node::TextType::TextItalic => ("\\textit{".to_string(),1),
        &node::TextType::TextMonospace => ("\\texttt{".to_string(),1),
        &node::TextType::TextBoldItalic => ("\\textit{\\textbf{".to_string(),2),
        &node::TextType::TextSansSerif => ("\\textsf{".to_string(),1),
        &node::TextType::TextSansSerifBold => ("\\textbf{\\textsf{".to_string(),2),
        &node::TextType::TextSansSerifItalic => ("\\textit{\\textsf{".to_string(),2),
        &node::TextType::TextSansSerifBoldItalic => ("\\textbf{\\textit{\\textsf{".to_string(),3),
        _ => ("\\text{".to_string(),1),
    }
}

fn xarrow(e: &node::Exp) -> Option<String>{
    // TODO: 将Symbol Op转换为对应的xarrow控制序列
    panic!("xarrow not implemented");
}

// TODO: what is fancy
fn is_fancy(e: &node::Exp) -> bool{
    match e{
        &node::Exp::ESub(..) => true,
        &node::Exp::ESuper(..) => true,
        &node::Exp::ESubsup(..) => true,
        &node::Exp::EUnder(..) => true,
        &node::Exp::EOver(..) => true,
        &node::Exp::EUnderOver(..) => true,
        &node::Exp::ERoot(..) => true,
        &node::Exp::ESqrt(..) => true,
        &node::Exp::EPhantom(..) => true,
        _ => false,
    }
}

fn is_operator(e: &node::Exp) -> bool{
    match e{
        &node::Exp::ESymbol(node::TeXSymbolType::Op, ..) => true,
        &node::Exp::EMathOperator(..) => true,
        _ => false,
    }
}

#[test]
fn test_write_etext(){
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let mut s = String::new();

    s.clear();
    let case = Exp::EText(node::TextType::TextNormal, "abc".to_string());
    write_tex(&case, &mut s, &envs);
    assert_eq!(s, "\\text{abc}");

    s.clear();
    let case = Exp::EText(node::TextType::TextBold, "abc".to_string());
    write_tex(&case, &mut s, &envs);
    assert_eq!(s, "\\textbf{abc}");

    s.clear();
    let case = Exp::EText(node::TextType::TextSansSerifBoldItalic, "abc".to_string());
    write_tex(&case, &mut s, &envs);
    assert_eq!(s, "\\textbf{\\textit{\\textsf{abc}}}");
}
fn write_tex(exp: &node::Exp, res: &mut String, envs: &HashMap<String, bool>) {
    match exp{
        node::Exp::ENumber(n) => {
            res.push_str(n);
        },

        node::Exp::EBoxed(exp) => {
            if envs["amsmath"]{
                res.push_str("\\boxed{");
                write_tex(exp,res, envs);
                res.push_str("}");
            }else{
                write_tex(exp,res, envs);
            }
        },

        node::Exp::EGrouped(exp_list) => {
            res.push_str("{");
            for exp in exp_list{
                write_tex(exp,res, envs);
            }
            res.push_str("}");
        },

        node::Exp::EDelimited(left, right, exp_list) => {
            let flag = delimited_fraction_noline(res, left, right, exp_list, envs);
            if flag{
                return;
            }

            let flag = delimited_write_right_array(res, left, right, exp_list, envs);
            if flag {
                return;
            }

            // general
            delimited_write_general_exp(res, left, right, exp_list, envs);
        },

        node::Exp::ESymbol(symbol_type, symbol) => {
            // writeExp (ESymbol Ord (T.unpack -> [c]))  -- do not render "invisible operators"
            //   | c `elem` ['\x2061'..'\x2064'] = return () -- see 3.2.5.5 of mathml spec

            if symbol_type == &node::TeXSymbolType::Ord && symbol.len() == 1{
                let c = symbol.chars().next().unwrap();
                if c >= '\u{2061}' && c <= '\u{2064}'{
                    return;
                }
            }


            let escaped = get_tex_math_many(&symbol, envs);
            // 如果是Bin, Rel则需要添加一个空格
            if *symbol_type == node::TeXSymbolType::Bin || *symbol_type == node::TeXSymbolType::Rel{
                res.push_str(" ");
            }

            res.push_str(&escaped);
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
            if *symbol_type == node::TeXSymbolType::Bin || *symbol_type == node::TeXSymbolType::Rel{
                res.push_str(" ");
            }
        },

        // ok
        node::Exp::ESpace(rational) => {
            let width = rational.numerator as f32 / rational.denominator as f32 * 18.0;
            let width = width.floor() as i32;
            match width {
                -3 => {
                    res.push_str("\\!");
                },
                0 => {}
                3 => {
                    res.push_str("\\, ");
                },
                4 => {
                    // use: \\  \\: \\>
                    res.push_str("\\ ");
                },
                5 => {
                    res.push_str("\\;");
                },
                18 => {
                    // TODO: ESpace: why \quad and \qquad need a space?
                    res.push_str("\\quad ");
                },
                36 => {
                    res.push_str("\\qquad ");
                },
                n => {
                    if envs["amsmath"]{
                        res.push_str("\\mspace{");
                        res.push_str(&n.to_string());
                        res.push_str("mu}");
                    }else{
                        res.push_str("\\mskip{");
                        res.push_str(&n.to_string());
                        res.push_str("mu}");
                    }
                }
            }
        },

        node::Exp::EIdentifier(identifier) => {
            res.push_str(&identifier);
        },

        node::Exp::EMathOperator(math_operator) => {
            // TODO: more precise MathOperator
            res.push_str("\\");
            res.push_str(&math_operator);
        },

        node::Exp::ESub(exp1, exp2) => {
            if is_fancy(exp1){
                res.push_str("{");
                write_tex(exp1,res, envs);
                res.push_str("}");
            }else{
                write_tex(exp1,res, envs);
            }

            res.push_str("_{");
            write_tex(exp2,res, envs);
            res.push_str("}");
        },

        node::Exp::ESuper(exp1, exp2) => {
            if is_fancy(exp1){
                res.push_str("{");
                write_tex(exp1,res, envs);
                res.push_str("}");
            }else{
                write_tex(exp1,res, envs);
            }

            res.push_str("^{");
            write_tex(exp2,res, envs);
            res.push_str("}");
        },

        node::Exp::ESubsup(exp1, exp2, exp3) => {
            if is_fancy(exp1){
                res.push_str("{");
                write_tex(exp1,res, envs);
                res.push_str("}");
            }else{
                write_tex(exp1,res, envs);
            }

            res.push_str("_{");
            write_tex(exp2,res, envs);
            res.push_str("}^{");
            write_tex(exp3,res, envs);
            res.push_str("}");
        },

        node::Exp::ESqrt(exp) => {
            res.push_str("\\sqrt");
            res.push_str("{");
            write_tex(exp,res, envs);
            res.push_str("}");
        },

        node::Exp::EFraction(fraction_type, exp1, exp2) => {
            res.push_str("\\");
            res.push_str(&fraction_type.to_str());
            res.push_str("{");
            write_tex(exp,res, envs);
            res.push_str("}{");
            write_tex(exp2,res, envs);
            res.push_str("}");
        },

        node::Exp::EText(text_type, str) => {
            if str.len() == 0{
                return;
            }
            let (cmd, repeats) = get_text_cmd(text_type);
            res.push_str(&cmd);
            res.push_str(&get_tex_math_many(str, envs));
            res.push_str("}".repeat(repeats as usize).as_str());
        },

        node::Exp::EStyled(text_type, exp_list) => {
            let cmd = get_style_latex_cmd(text_type, envs);
            res.push_str(cmd.as_str());
            res.push_str("{");
            for exp in exp_list{
                write_tex(exp, res, envs);
            }
            res.push_str("}");
        },

        node::Exp::EPhantom(exp) => {
            res.push_str("\\phantom{");
            write_tex(exp,res, envs);
            res.push_str("}");
        },

        node::Exp::EArray(alignments, exp_lists) => {
            if aligns_is_rlsequence(alignments){
                if envs["amsmath"]{
                    write_array_table(res, "aligned",&Vec::<node::Alignment>::new(), exp_lists, envs);
                    return;
                }else{
                    write_array_table(res, "array", alignments, exp_lists, envs);
                    return;
                }
            }

            if envs["amsmath"] && aligns_is_all_center(alignments){
                write_array_table(res, "matrix", &Vec::<node::Alignment>::new(), exp_lists, envs);
                return;
            }else{
                write_array_table(res, "array", alignments, exp_lists, envs);
                return;
            }
        },

        node::Exp::EOver(convertible, b, e1) => {
            match xarrow(b){
                Some(exp) => {
                    if envs["amsmath"]{
                        res.push_str(exp.as_str());
                        res.push_str("{");
                        write_tex(e1,res, envs);
                        res.push_str("}");
                    }
                },
                None => {
                    write_script(res, &Position::Over, convertible, b,e1);
                }
            };
        },

        node::Exp::EUnder(convertible, base, e1) => {
            write_script(res, &Position::Under, convertible, base, e1);
        },

        node::Exp::EUnderOver(convertible, b, e1, e2) => {
            if write_underover_accent(res, exp, envs){
                return;
            }

            match xarrow(b){
                Some(e) =>{
                    if envs["amsmath"]{
                        res.push_str(e.as_str());
                        res.push_str("[{");
                        write_tex(e1, res, envs);
                        res.push_str("}]{");
                        write_tex(e2, res, envs);
                        res.push_str("}");
                        return;
                    }
                }
                None => {
                    if is_operator(b){
                        if is_fancy(b){
                            res.push_str("{");
                            write_tex(b, res, envs);
                            res.push_str("}");
                        }else{
                            // TODO: 可能要增加convertible对write_tex的影响
                            if *convertible{
                                write_tex(b, res, envs);
                            }else{
                                res.push_str("\\limits");
                            }
                            res.push_str("_{");
                            check_substack(res, e1, envs);
                            res.push_str("}^{");
                            check_substack(res, e2, envs);
                            res.push_str("}");
                        }
                        return;
                    }
                }
            }
            // TODO: underover
            // writeExp (EUnder convertible (EOver convertible b e2) e1)
            panic!("writeExp (EUnder convertible (EOver convertible b e2) e1) not implemented");
        },

        node::Exp::ERoot(exp1, exp2) => {
            res.push_str("\\sqrt[");
            write_tex(exp1,res, envs);
            res.push_str("]");
            write_tex(exp2,res, envs);
        },

        node::Exp::EScaled(size, e) => {
            let flag = match **e {
                node::Exp::ESymbol(node::TeXSymbolType::Open, _) => true,
                node::Exp::ESymbol(node::TeXSymbolType::Close, _) => true,
                _ => false,
            };
            if flag{
                if let Some(cmd) = get_scaler_cmd(size){
                    res.push_str(cmd.as_str());
                }
                write_tex(e, res, envs);
            }else{
                write_tex(e, res, envs);
            }
        },
    }
}

// impl ToTeX for node::ExpList{
//     fn to_show(&self, envs: &HashMap<String, bool>) -> String{
//         let mut s = String::new();
//         if self.list.len() == 0{
//             return s;
//         }else if self.list.len() == 1{
//             return self.list[0].write_tex(envs);
//         }

//         s.push_str("{");
//         for exp in &self.list{
//             s.push_str(&exp.write_tex(envs));
//         }
//         s.push_str("}");
//         s
//     }
// }

impl node::Rational{
    fn to_show(&self) -> String{
        let mut s = String::new();
        s.push_str(&self.numerator.to_string());
        s.push_str("/");
        s.push_str(&self.denominator.to_string());
        s
    }
}

impl node::Alignment{
    fn to_str(&self) -> String{
        match self{
            node::Alignment::AlignLeft => {
                "l".to_string()
            },

            node::Alignment::AlignRight => {
                "r".to_string()
            },

            node::Alignment::AlignCenter => {
                "c".to_string()
            },
        }
    }
}

impl node::FractionType{
    fn to_str(&self) -> String{
        match self{
            node::FractionType::NormalFrac => {
                "frac".to_string()
            },

            node::FractionType::DisplayFrac => {
                "dfrac".to_string()
            },

            node::FractionType::InlineFrac => {
                "tfrac".to_string()
            },

            node::FractionType::NoLineFrac => {
                "binom".to_string()
            },
        }
    }
}


impl node::TeXSymbolType{
    fn to_show(&self) -> String{
        match self{
            node::TeXSymbolType::Bin => {
                "bin".to_string()
            },
            node::TeXSymbolType::Rel => {
                "rel".to_string()
            },
            node::TeXSymbolType::Open => {
                "open".to_string()
            },
            _ => {
                panic!("others to_show not implemented");
            },
        }
    }
}