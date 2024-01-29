use std::collections::HashMap;

use super::node;
use super::env_unicode;

pub struct TexWriter{
    e : node::Exp,
    envs : HashMap<String, bool>,
}

impl TexWriter {
    pub fn new_exp(e: node::Exp, envs: HashMap<String, bool>) -> Self {
        TexWriter {
            e: e,
            envs: envs,
        }
    }

    pub fn to_tex(&self) -> String{
        let tex = self.e.to_tex(&self.envs);
        // 除去首位{}
        let tex = tex[1..tex.len()-1].to_string();
        tex
    }
}
pub trait ToTeX {
    fn to_show(&self, envs: &HashMap<String, bool>) -> String;
}

impl node::Exp{
    fn to_tex(&self, envs: &HashMap<String, bool>) -> String{
        match self{
            node::Exp::ENumber(n) => {
                n.to_string()
            },

            node::Exp::ExpList(list) => {
                let mut s = String::new();
                s.push_str("{");
                for exp in list{
                    s.push_str(&exp.to_tex(envs));
                }
                s.push_str("}");
                s
            },

            node::Exp::EBoxed(exp) => {
                let mut s = String::new();
                s.push_str("\\boxed{");
                s.push_str(&exp.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::EGrouped(exp_list) => {
                exp_list.to_show(envs)
            },

            node::Exp::EDelimited(left, right, exp_list) => {
                let mut s = String::new();
                s.push_str("\\left");
                s.push_str(&left);
                s.push_str(&exp_list.to_show(envs));
                s.push_str("\\right");
                s.push_str(&right);
                s
            },

            node::Exp::ESymbol(symbol_type, symbol) => {
                let mut s = String::new();
                let escaped = env_unicode::escape_symbol(&symbol, envs);
                // 如果是Bin, Rel则需要添加一个空格
                if *symbol_type == node::TeXSymbolType::Bin || *symbol_type == node::TeXSymbolType::Rel{
                    s.push_str(" ");
                }

                s.push_str(&escaped);
                // todo
                // if symbol.len() > 1 && (symbol_type == &node::TeXSymbolType::Bin || symbol_type == &node::TeXSymbolType::Rel || symbol_type == &node::TeXSymbolType::Op) {
                //     s.push_str("\\math");
                //     s.push_str(symbol_type.to_show().as_str()); // todo
                //     s.push_str("{");
                    
                //     s.push_str("\\text{");
                //     s.push_str(&escaped);
                //     s.push_str("}");

                //     s.push_str("}");
                // }

                // 如果是Bin, Rel则需要添加一个空格
                if *symbol_type == node::TeXSymbolType::Bin || *symbol_type == node::TeXSymbolType::Rel{
                    s.push_str(" ");
                }
                s
            },

            // ok
            node::Exp::ESpace(rational) => {
                let mut s = String::new();
                let width = rational.numerator as f32 / rational.denominator as f32 * 18.0;
                let width = width.floor() as i32;
                match width {
                    -3 => {
                        s.push_str("\\!");
                    },
                    0 => {}
                    3 => {
                        s.push_str("\\,");
                    },
                    4 => {
                        // use: \\  \\: \\>
                        s.push_str("\\ ");
                    },
                    5 => {
                        s.push_str("\\;");
                    },
                    18 => {
                        s.push_str("\\quad");
                    },
                    36 => {
                        s.push_str("\\qquad");
                    },
                    n => {
                        if envs["amsmath"]{
                            s.push_str("\\mspace{");
                            s.push_str(&n.to_string());
                            s.push_str("mu}");
                        }else{
                            s.push_str("\\mskip{");
                            s.push_str(&n.to_string());
                            s.push_str("mu}");
                        }
                    }
                }
                s
            },

            node::Exp::EIdentifier(identifier) => {
                identifier.to_string()
            },

            node::Exp::EMathOperator(math_operator) => {
                let mut s = String::new();
                s.push_str("\\");
                s.push_str(&math_operator);
                s
            },

            node::Exp::ESub(exp1, exp2) => {
                let mut s = String::new();
                s.push_str(&exp1.to_tex(envs));
                s.push_str("_{");
                s.push_str(&exp2.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::ESuper(exp1, exp2) => {
                let mut s = String::new();
                s.push_str(&exp1.to_tex(envs));
                s.push_str("^{");
                s.push_str(&exp2.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::ESubsup(exp1, exp2, exp3) => {
                let mut s = String::new();
                s.push_str(&exp1.to_tex(envs));
                s.push_str("_{");
                s.push_str(&exp2.to_tex(envs));
                s.push_str("}^{");
                s.push_str(&exp3.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::ESqrt(exp) => {
                let mut s = String::new();
                s.push_str("\\sqrt");
                s.push_str(&exp.to_tex(envs));
                s
            },

            node::Exp::EFraction(fraction_type, exp1, exp2) => {
                let mut s = String::new();
                s.push_str("\\");
                s.push_str(&&fraction_type.to_show());
                s.push_str(&exp1.to_tex(envs));
                s.push_str(&exp2.to_tex(envs));
                s
            },

            node::Exp::EText(text_type, str) => {
                let mut s = String::new();
                s.push_str("\\");
                s.push_str(&text_type.to_show());
                s.push_str("{");
                s.push_str(&str);
                s.push_str("}");
                s
            },

            node::Exp::EStyled(text_type, exp_list) => {
                let mut s = String::new();
                s.push_str("\\");
                s.push_str(&text_type.to_show());
                s.push_str("{");
                s.push_str(&exp_list.to_show(envs));
                s.push_str("}");
                s
            },

            node::Exp::EPhantom(exp) => {
                let mut s = String::new();
                s.push_str("\\phantom{");
                s.push_str(&exp.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::EArray(alignments, exp_lists) => {
                let mut s = String::new();
                s.push_str("\\begin{array}{");
                for alignment in alignments{
                    s.push_str(&alignment.to_show());
                }
                s.push_str("}");
                for exp_list in exp_lists{
                    for exp in exp_list{
                        s.push_str(&exp.to_show(envs));
                    }
                }
                s.push_str("\\end{array}");
                s
            },

            node::Exp::EOver(is_over, exp1, exp2) => {
                let mut s = String::new();
                if *is_over{
                    s.push_str("\\overline{");
                }else{
                    s.push_str("\\underline{");
                }
                s.push_str(&exp1.to_tex(envs));
                s.push_str("}");
                s.push_str(&exp2.to_tex(envs));
                s
            },

            node::Exp::EUnder(is_under, exp1, exp2) => {
                let mut s = String::new();
                if *is_under{
                    s.push_str("\\overline{");
                }else{
                    s.push_str("\\underline{");
                }
                s.push_str(&exp1.to_tex(envs));
                s.push_str("}");
                s.push_str(&exp2.to_tex(envs));
                s
            },

            node::Exp::EUnderOver(is_under_over, exp1, exp2, exp3) => {
                let mut s = String::new();
                if *is_under_over{
                    s.push_str("\\overline{");
                }else{
                    s.push_str("\\underline{");
                }
                s.push_str(&exp1.to_tex(envs));
                s.push_str("}");
                s.push_str(&exp2.to_tex(envs));
                s.push_str(&exp3.to_tex(envs));
                s
            },


            node::Exp::ERoot(exp1, exp2) => {
                let mut s = String::new();
                s.push_str("\\sqrt[");
                s.push_str(&exp1.to_tex(envs));
                s.push_str("]");
                s.push_str(&exp2.to_tex(envs));
                s
            },

            node::Exp::EScaled(rational, exp) => {
                let mut s = String::new();
                s.push_str("\\scalebox{");
                s.push_str(&rational.to_show());
                s.push_str("}{");
                s.push_str(&exp.to_tex(envs));
                s.push_str("}");
                s
            },

            node::Exp::Right(exp) => {
                let mut s = String::new();
                s.push_str("\\right");
                s.push_str(&exp.to_tex(envs));
                s
            },

            node::Exp::Left(str) => {
                let mut s = String::new();
                s.push_str("\\left");
                s.push_str(&str);
                s
            },
        }
    }
}

impl ToTeX for node::ExpList{
    fn to_show(&self, envs: &HashMap<String, bool>) -> String{
        let mut s = String::new();
        if self.list.len() == 0{
            return s;
        }else if self.list.len() == 1{
            return self.list[0].to_tex(envs);
        }

        s.push_str("{");
        for exp in &self.list{
            s.push_str(&exp.to_tex(envs));
        }
        s.push_str("}");
        s
    }
}

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
                // error todo
                "".to_string()
            },
        }
    }
}


#[test]
fn test_totex_number(){
    let exp = node::Exp::ENumber("123".to_string());
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), "123");
}

#[test]
fn test_totex_fraction(){
    let exp = node::Exp::EFraction(
        node::FractionType::NormalFrac, 
        Box::new(node::Exp::EFraction(
            node::FractionType::NormalFrac, 
            Box::new(node::Exp::ENumber("1".to_string())), 
            Box::new(node::Exp::ENumber("2".to_string()))
        )),
        Box::new(node::Exp::ENumber("2".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\frac{\frac{1}{2}}{2}"#);
}

#[test]
fn test_totex_sqrt(){
    let exp = node::Exp::ESqrt(
        Box::new(node::Exp::ENumber("123".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\sqrt{123}"#);
}

#[test]
fn test_totex_root(){
    let exp = node::Exp::ERoot(
        Box::new(node::Exp::ENumber("2".to_string())),
        Box::new(node::Exp::ENumber("123".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\sqrt[2]{123}"#);
}

#[test]
fn test_totex_identifier(){
    let exp = node::Exp::EIdentifier("x".to_string());
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), "x");
}

#[test]
fn test_totex_symbol(){
    todo!("symbol");
}

#[test]
fn test_totex_boxed(){
    let exp = node::Exp::EBoxed(
        Box::new(node::Exp::ENumber("123".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\boxed{123}"#);
}

#[test]
fn test_totex_under_and_over(){
    let exp = node::Exp::EUnder(
        true,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\overline{123}456"#);

    let exp = node::Exp::EOver(
        true,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\overline{123}456"#);

    let exp = node::Exp::EUnderOver(
        true,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())),
        Box::new(node::Exp::ENumber("789".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\overline{123}456789"#);

    let exp = node::Exp::EUnder(
        false,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\underline{123}456"#);

    let exp = node::Exp::EOver(
        false,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\underline{123}456"#);

    let exp = node::Exp::EUnderOver(
        false,
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())),
        Box::new(node::Exp::ENumber("789".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), r#"\underline{123}456789"#);
}

#[test]
fn test_totex_super_sub(){
    let exp = node::Exp::ESub(
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), "123_{456}");

    let exp = node::Exp::ESuper(
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), "123^{456}");

    let exp = node::Exp::ESubsup(
        Box::new(node::Exp::ENumber("123".to_string())),
        Box::new(node::Exp::ENumber("456".to_string())),
        Box::new(node::Exp::ENumber("789".to_string())));
    let tr = TexWriter::new_exp(exp, HashMap::<String, bool>::new());
    assert_eq!(tr.to_tex(), "123_{456}^{789}");
}
