use nom::AsChar;

// 渲染的序列
#[derive(PartialEq, Debug)]
pub enum Tex {
    ControlSeq(String),
    Token(char), // 控制字符
    Literal(String), // 字符串字面量
    Grouped(Vec<Tex>),
    Space,
}

// #[test]
// fn test_render_tex() {
//     let tex = Tex::Grouped(vec![
//         Tex::ControlSeq("frac".to_string()),
//         Tex::Grouped(vec![
//             Tex::ControlSeq("sin".to_string()),
//             Tex::Grouped(vec![
//                 Tex::ControlSeq("theta".to_string()),
//             ]),
//         ]),
//         Tex::Grouped(vec![
//             Tex::ControlSeq("cos".to_string()),
//             Tex::Grouped(vec![
//                 Tex::ControlSeq("theta".to_string()),
//             ]),
//         ]),
//     ]);
//     assert_eq!(render_tex(&tex), "{\\frac{\\sin{\\theta}}{\\cos{\\theta}}}");
// }

// // 渲染
// pub fn render_tex(tex: &Tex) -> String {
//     let mut res = String::new();
//     match tex {
//         Tex::ControlSeq(s) => {
//             // | s == "\\ "               = s <> cs
//             // | startsWith (\c -> isAlphaNum c || not (isAscii c)) cs
//             //                            = s <> T.cons ' ' cs
//             // | otherwise                = s <> cs
//             if s == "\\ " {
//                 res.push_str(&s);
//             } else if s.starts_with(|c: char| c.is_alphanumeric() || !c.is_ascii()) {
//                 // 如果字符串以字母或非ASCII字符开头，则在中间加一个空格
//                 res.push_str(&s);
//                 res.push(' ');
//             } else {
//                 res.push_str(&s);
//             }
//         }
//         Tex::Token(c) => {
//             res.push(*c);
//         }
//         Tex::Literal(s) => {
//             // | endsWith (not . isLetter) s = s <> cs
//             // | startsWith isLetter cs      = s <> T.cons ' ' cs
//             // | otherwise                   = s <> cs
//             if s.ends_with(|c: char| !c.is_alphabetic()) {
//                 // 如果字符串以非字母字符结尾，则直接拼接
//                 res.push_str(&s);
//             } else if s.starts_with(|c: char| c.is_alphabetic()) {
//                 // 如果字符串以字母字符开头，则在中间加一个空格
//                 // 这里的空格是为了避免两个字母字符连在一起
//                 res.push_str(&s);
//                 res.push(' ');
//             } else {
//                 // 其他情况直接拼接
//                 res.push_str(&s);
//             }
//         }
//         Tex::Grouped(v) => {
//             // 如果是Grouped[Grouped[...]]，则去掉一层Grouped
//             if v.len() == 1 && matches!(v[0], Tex::Grouped(_)) {
//                 return render_tex(&v[0]);
//             }
//
//             //   "{" <> foldr renderTeX "" (trimSpaces xs) <> "}" <> cs
//             res.push('{');
//             for t in v {
//                 res.push_str(&render_tex(t));
//             }
//             res.push('}');
//         }
//         Tex::Space => {
//             // | cs == ""                   = ""
//             //     | any (`T.isPrefixOf` cs) ps = cs
//             //     | otherwise                  = T.cons ' ' cs
//             // where
//             // -- No space before ^, _, or \limits, and no doubled up spaces
//             // ps = [ "^", "_", " ", "\\limits" ]
//
//             // 如果字符串为空，则返回空字符串
//             // 如果字符串中包含 ^, _, 空格, \limits，则返回原字符串
//             // 其他情况在字符串前面加一个空格
//             return if res.is_empty() {
//                 res
//             } else if res.contains('^') || res.contains('_') || res.contains(' ') || res.contains("\\limits") {
//                 res
//             } else {
//                 " ".to_string() + &res
//             }
//         }
//     }
//
//     res
// }