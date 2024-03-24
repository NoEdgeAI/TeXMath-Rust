use crate::ast::node::Exp;
use crate::ast::node::TextType::TextNormal;

// TODO: 转换AI输出的AST -> 标准并进行错误修正

#[test]
fn test_read_ast(){
    let ast = r#"
    <TAB0|>[<TAB1|>ENumber "213",ESpace {1 % 1},ENumber "1",ESpace {1 % 1}
    "#;
    let exp = read_ast(ast).unwrap();
    assert_eq!(exp, Exp::EText(TextNormal, "Hello".to_string()));
}

pub fn read_ast(ast: &str) -> Result<Exp, String>{
    // TODO: 读取AST
    Ok(Exp::EText(TextNormal, "Hello".to_string()))
}