use std::fs;
use std::path::Path;
use std::io;

mod ast;
use std::time::Instant;

use crate::ast::parser::parse_exp;
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


fn main() -> std::io::Result<()> {
    let dir = "./src/tex"; // 使用当前目录，你可以改为任意目录路径
    let files = read_dir_files_to_vec(Path::new(dir))?; // 读取目录下所有文件

    println!("{} files found", files.len());
    

    let mut i = 0;

    let now = Instant::now();
    for file in files {
        let res = parse_exp(&file);
        // if has error
        if let Err(e) = res {
            println!("====================");
            println!("File: {}", i);
            println!("{}", file);
            println!("Error: {}", e);
            println!("====================");
            return Ok(());
        }else{
            // if no error
            i += 1;
        }
    }

    println!("{} files parsed successfully", i);
    println!("Time elapsed: {}ms", now.elapsed().as_millis());

    Ok(())
}