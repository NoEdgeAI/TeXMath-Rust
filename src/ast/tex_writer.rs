use core::panic;
use std::collections::HashMap;

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
    [ EText
    TextNormal
    "Inline double up down arrows, text style, thick line\160"
, EDelimited
    ""
    ""
    [ Left "\8661"
    , Right
        (EFraction
           NormalFrac
           (EGrouped
              [ EMathOperator "sin" , ESymbol Ord "\8289" , EIdentifier "\952" ])
           (EIdentifier "M"))
    , Left "\8661"
    ]
, EText TextNormal "\160the end."
]"#;
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    let exp = super::ast_reader::read_ast(case).unwrap();
    dbg!(&exp);
    let tex_writer: TexWriter = TexWriter::new_exp(exp, envs);
    let tex = tex_writer.to_tex();
    println!("{}", tex);
}

impl TexWriter {
    pub fn new_exp(e: Vec<node::Exp>, envs: HashMap<String, bool>) -> Self {
        TexWriter {
            e: e,
            envs: envs,
        }
    }

    pub fn to_tex(&self) -> String{
        let mut s = String::new();
        
        for exp in &self.e{
            exp.write_tex(&mut s, &self.envs);
        }
        s
    }
}

pub trait ToTeX {
    fn to_show(&self, envs: &HashMap<String, bool>) -> String;
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
            node::InEDelimited::Middle(..) => {
                return false;
            },
            node::InEDelimited::Exp(exp) => {
                match exp {
                    Exp::Right(..) => {},
                    _ => {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

// get tex math m
// TODO escape get tex math m
fn get_tex_math_m(open: &str, envs: &HashMap<String, bool>) -> String{
    let open = to_tex_unicode::escape_symbol_unicode(open, envs);
    return String::from(open);
}

#[test]
fn test_remove_outer_group(){
    let test_case = Exp::EGrouped(vec![
            Exp::Right(Box::new(Exp::ENumber("5".to_string()))),
        ]
    );

    let res = remove_outer_group(&test_case);
    assert_eq!(res, &Exp::Right(Box::new(Exp::ENumber("5".to_string()))));

    let test_case = Exp::EGrouped(vec![
            Exp::Right(Box::new(Exp::ENumber("5".to_string()))),
            Exp::Right(Box::new(Exp::ENumber("6".to_string()))),
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
    // [ Right (ENumber "5")
    // , Right (ESymbol Bin "\8722")
    // , Right (EIdentifier "T")
    // ]
    let test_case = vec![
        node::InEDelimited::Exp(Exp::Right(Box::new(Exp::ENumber("5".to_string())))),
        node::InEDelimited::Exp(Exp::Right(Box::new(Exp::ESymbol(node::TeXSymbolType::Bin, "\\8722".to_string())))),
        node::InEDelimited::Exp(Exp::Right(Box::new(Exp::EIdentifier("T".to_string())))),
        ];

    assert_eq!(is_all_standard_height(&test_case), true);
}

// check if all exp is standard height:
// Right(ENumber, EIdentifier, ESpace, ESymbol(Ord, Op, Bin, Rel, Pun))
fn is_all_standard_height(exp: &Vec<node::InEDelimited>) -> bool{
    for e in exp{
        match e {
            node::InEDelimited::Middle(..) => {
                panic!("is_all_standard_height: middle not implemented");
            },
            node::InEDelimited::Exp(exp) => {
                match exp {
                    Exp::Right(exp) => {
                        match exp.as_ref(){
                            Exp::ENumber(..) => {},
                            Exp::EIdentifier(..) => {},
                            Exp::ESpace(..) => {},
                            Exp::ESymbol(symbol_type, ..) => {
                                match symbol_type{
                                    node::TeXSymbolType::Ord => {},
                                    node::TeXSymbolType::Op => {},
                                    node::TeXSymbolType::Bin => {},
                                    node::TeXSymbolType::Rel => {},
                                    node::TeXSymbolType::Pun => {},
                                    _ => {
                                        return false;
                                    }
                                }
                            },
                            _ => {
                                return false;
                            }
                        }
                    },
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
        exp1.write_tex(s, envs);
        s.push_str("}{");
        exp2.write_tex(s, envs);
        s.push_str("}");
    }
}

// 输出alignments, 不带{}
// AlignLeft -> l, AlignRight -> r, AlignCenter -> c
fn write_alignments(s: &mut String, aligns: &Vec<node::Alignment>){
    for align in aligns{
        s.push_str(&align.to_show());
    }
}

#[test]
fn test_write_arraylines(){
    let case = vec![
        vec![
            vec![Exp::Right(Box::new(Exp::ENumber("1".to_string())))],
            vec![Exp::Right(Box::new(Exp::ENumber("2".to_string())))],
        ],
        vec![
            vec![Exp::Right(Box::new(Exp::ENumber("3".to_string())))],
            vec![Exp::Right(Box::new(Exp::ENumber("4".to_string())))],
        ],
    ];

    let mut s = String::new();
    write_arraylines(&mut s, &case, &HashMap::new());
    assert_eq!(s, "1 & 2 \\\\\n3 & 4 \\\\\n");
}

// 输出array的单个元素
fn write_arrayline(s: &mut String, row: &Vec<node::Exp>, envs: &HashMap<String, bool>){
    if row.len() == 0{
        return;
    }else if row.len() == 1{
        row[0].write_tex(s, envs);
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

// TODO neat this function
fn delimited_write_right_array(s: &mut String, left: &String, right: &String, exp: &Vec<node::InEDelimited>, envs: &HashMap::<String, bool>) -> bool {
    if exp.len() != 1{
        return false;
    }
    let exp = &exp[0];
    match exp{
        node::InEDelimited::Middle(..) => {
            panic!("delimited in middle not implemented");
        },
        node::InEDelimited::Exp(exp) => {
            if let Exp::Right(ref exp) = exp{
                if let Exp::EArray(ref aligns, ref rows) = exp.as_ref(){
                    if envs["amsmath"]{
                        if(left.as_str() == "{" && right.as_str() == "") || aligns == &[node::Alignment::AlignLeft, node::Alignment::AlignLeft]{
                            // TODO delimited write array table
                            panic!("delimited_write_right_array not implemented");
                        }
                    }
                }
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
        node::InEDelimited::Middle(_) => {
            panic!("delimited in middle not implemented");
        },
        node::InEDelimited::Exp(exp) => {
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
                        // TODO delimited write right array
                        panic!("delimited_fraction_noline not implemented");
                    }
                }
            }
        }
    }
    return false;
}
enum FenceType{
    DLeft(),
    DMiddle(),
    DRight(),
}
fn delimited_write_delim(s: &mut String, ft: FenceType, delim: &str){
    // TODO: escape string
    // TODO: valid delim
    panic!("delimited_write_delim not implemented");
}
fn delimited_write_general_exp(s: &mut String, left: &String, right: &String, exp_list: &Vec<node::InEDelimited>, envs: &HashMap::<String, bool>){
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
    match (left.as_str(), right.as_str()){
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

        s.push_str(&get_tex_math_m(left, envs));
        // TODO delimited general exp
        // mapM_ (either (writeDelim DMiddle) writeExp) es
        for exp in exp_list{
            match exp {
                node::InEDelimited::Middle(..) => {
                    panic!("delimited in middle not implemented");
                },
                node::InEDelimited::Exp(exp) => {
                    exp.write_tex(s, envs);
                }      
            }
        }
        s.push_str(&get_tex_math_m(right, envs));
    }

    // write_delim
    delimited_write_delim(s, FenceType::DLeft(), left);
    for exp in exp_list{
        match exp {
            node::InEDelimited::Middle(..) => {
                delimited_write_delim(s, FenceType::DMiddle(), delim);
            },
            node::InEDelimited::Exp(exp) => {
                exp.write_tex(s, envs);
            }      
        }
    }
}

impl node::Exp{
    fn write_tex(&self, res: &mut String, envs: &HashMap<String, bool>) {
        match self{
            node::Exp::ENumber(n) => {
                res.push_str(n);
            },

            node::Exp::EBoxed(exp) => {
                res.push_str("\\boxed{");
                exp.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::EGrouped(exp_list) => {
                res.push_str("{");
                for exp in exp_list{
                    exp.write_tex(res, envs);
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


                let escaped = get_tex_math_m(&symbol, envs);
                // 如果是Bin, Rel则需要添加一个空格
                if *symbol_type == node::TeXSymbolType::Bin || *symbol_type == node::TeXSymbolType::Rel{
                    res.push_str(" ");
                }

                res.push_str(&escaped);
                // TODO symbol escape
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
                        // TODO ESpace: why here is need a space?
                        res.push_str("\\quad ");
                    },
                    36 => {
                        // TODO ESpace: why here is need a space?
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
                res.push_str("\\");
                res.push_str(&math_operator);
            },

            node::Exp::ESub(exp1, exp2) => {
                exp1.write_tex(res, envs);
                res.push_str("_{");
                exp2.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::ESuper(exp1, exp2) => {
                exp1.write_tex(res, envs);
                res.push_str("^{");
                exp2.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::ESubsup(exp1, exp2, exp3) => {
                exp1.write_tex(res, envs);
                res.push_str("_{");
                exp2.write_tex(res, envs);
                res.push_str("}^{");
                exp3.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::ESqrt(exp) => {
                res.push_str("\\sqrt");
                res.push_str("{");
                exp.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::EFraction(fraction_type, exp1, exp2) => {
                res.push_str("\\");
                res.push_str(&fraction_type.to_show());
                res.push_str("{");
                exp1.write_tex(res, envs);
                res.push_str("}{");
                exp2.write_tex(res, envs);
                res.push_str("}");
            },

            node::Exp::EText(text_type, str) => {
                res.push_str("\\");
                res.push_str(&text_type.to_show());
                res.push_str("{");
                res.push_str(&str);
                res.push_str("}");
            },

            node::Exp::EStyled(text_type, exp_list) => {
                res.push_str("\\");
                res.push_str(&text_type.to_show());
                self.write_tex(res, envs);
            },

            node::Exp::EPhantom(exp) => {
                res.push_str("\\phantom");
                res.push_str("{");
                exp.write_tex(res, envs);
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

            node::Exp::EOver(is_over, exp1, exp2) => {
                // TODO write exp over
                panic!("EOver not implemented");
            },

            node::Exp::EUnder(is_under, exp1, exp2) => {
                // TODO write exp under
                panic!("EUnder not implemented");
            },

            node::Exp::EUnderOver(is_under_over, exp1, exp2, exp3) => {
                // TODO write exp under over
                panic!("EUnderOver not implemented");
            },


            node::Exp::ERoot(exp1, exp2) => {
                res.push_str("\\sqrt[");
                exp1.write_tex(res, envs);
                res.push_str("]");
                exp2.write_tex(res, envs);
            },

            node::Exp::EScaled(rational, exp) => {
                // TODO write exp scaled
                panic!("EScaled not implemented");
            },

            node::Exp::Right(exp) => {
                exp.write_tex(res, envs);
            },

            node::Exp::Left(str) => {
                // TODO write exp left
                panic!("Left not implemented");
            },
        }
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
    fn to_show(&self) -> String{
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
    fn to_show(&self) -> String{
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

impl node::TextType{
    fn to_show(&self) -> String{
        match self{
            node::TextType::TextNormal => {
                "text".to_string()
            },

            node::TextType::TextBold => {
                "textbf".to_string()
            },

            node::TextType::TextItalic => {
                "textit".to_string()
            },

            node::TextType::TextMonospace => {
                "texttt".to_string()
            },

            node::TextType::TextSansSerif => {
                "textsf".to_string()
            },

            node::TextType::TextDoubleStruck => {
                "textbb".to_string()
            },

            node::TextType::TextScript => {
                "textsc".to_string()
            },

            node::TextType::TextFraktur => {
                "textfrak".to_string()
            },

            node::TextType::TextBoldItalic => {
                "textbf".to_string()
            },

            node::TextType::TextSansSerifBold => {
                "textsf".to_string()
            },

            node::TextType::TextSansSerifBoldItalic => {
                "textsf".to_string()
            },

            node::TextType::TextBoldScript => {
                "textbf".to_string()
            },

            node::TextType::TextBoldFraktur => {
                "textbf".to_string()
            },

            node::TextType::TextSansSerifItalic => {
                "textsf".to_string()
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