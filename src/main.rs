use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io;

mod ast;
use std::time::Instant;

use crate::ast::ast_reader;

fn read_dir_files_to_vec(dir: &Path) -> io::Result<Vec<String>> {
    let mut file_contents = Vec::new();

    // 遍历目录
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // 只处理文件
        if path.is_file() {
            // 读取文件内容
            match fs::read_to_string(&path) {
                Ok(content) => {
                    // 处理文本内容，提取<<< native和>>> tex之间的内容
                    if let Some(start) = content.find("<<< native") {
                        if let Some(end) = content.find(">>> tex") {
                            // 截取开始和结束标记之间的内容
                            let extracted = &content[start + "<<< native".len()..end];
                            file_contents.push(extracted.trim().to_string());
                        }
                    }
                },
                Err(e) => return Err(e), // 如果读取文件失败，则返回错误
            }
        }
    }

    Ok(file_contents)
}

fn test_read_tex(){
    let dir = "./src/tex"; // 使用当前目录，你可以改为任意目录路径
    let files = read_dir_files_to_vec(Path::new(dir)); // 读取目录下所有文件

    println!("{} files found", files.as_ref().unwrap().len());
    

    let mut i = 0; 
    let files = files.unwrap();

    let now = Instant::now();
    for file in files {
        match ast_reader::read_ast(&file) {
            Ok (exp) => {
                // println!("Exp read successfully");
                // dbg!(exp);
                i += 1;
            },
            Err(e) => {
                println!("file: {}", file);
                println!("Parse error: {:?}", e);
                return;
            }
        }
    }

    println!("{} files parsed successfully", i);
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
}

fn test_totex(){
    let exp_str = r#"
    [ EUnderover
    True
    (ESymbol Op "\8721")
    (EGrouped [ EIdentifier "m" , ESymbol Rel "=" , ENumber "1" ])
    (ESymbol Ord "\8734")
, EUnderover
    True
    (ESymbol Op "\8721")
    (EGrouped [ EIdentifier "n" , ESymbol Rel "=" , ENumber "1" ])
    (ESymbol Ord "\8734")
, EFraction
    NormalFrac
    (EGrouped
       [ ESuper (EIdentifier "m") (ENumber "2")
       , ESpace (1 % 6)
       , EIdentifier "n"
       ])
    (EGrouped
       [ ESuper (ENumber "3") (EIdentifier "m")
       , EDelimited
           "("
           ")"
           [ Right (EIdentifier "m")
           , Right (ESpace (1 % 6))
           , Right (ESuper (ENumber "3") (EIdentifier "n"))
           , Right (ESymbol Bin "+")
           , Right (EIdentifier "n")
           , Right (ESpace (1 % 6))
           , Right (ESuper (ENumber "3") (EIdentifier "m"))
           ]
       ])
]
    "#;
    match ast::ast_reader::read_ast(exp_str){
        Ok(e) => {
            println!("Exp read successfully");
            let env = HashMap::<String, bool>::new();
            let tr = ast::tex_writer::TexWriter::new_exp(
                e, 
                env);
            println!("|{}|", tr.to_tex());
        },
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    
    }
}


fn main() -> std::io::Result<()> {
    test_totex();
    Ok(())
}