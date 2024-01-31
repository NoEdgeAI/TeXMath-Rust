use std::collections::HashMap;
use std::f32::consts::E;
use std::{fs, panic};
use std::path::Path;
use std::io;

mod ast;
use std::thread::panicking;
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
    let files: Vec<String> = files.unwrap();

    let now = Instant::now();
    for file in files {
        match ast_reader::read_ast(&file) {
            Ok (_) => {
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
fn read_dir_files(dir: &Path) -> io::Result<(Vec<String>, Vec<String>)> {
    let mut natives = Vec::new();
    let mut texs = Vec::new();

    // 遍历目录
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // 只处理文件
        if path.is_file() {
            // 读取文件内容
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if let Some(start) = content.find("<<< native") {
                        if let Some(end) = content.find(">>> tex") {
                            // 截取
                            // <<< native
                            // >>> tex
                            // 之间的内容
                            let extracted = &content[start + "<<< native".len()..end];
                            if extracted.len() == 0 {
                                panic!("empty native");
                            }
                            natives.push(extracted.trim().to_string());

                            // 从<<< native之后开始截取
                            let extracted = &content[end + ">>> tex".len()..];
                            if extracted.len() == 0 {
                                panic!("empty tex");
                            }
                            texs.push(extracted.trim().to_string());
                        }
                    }
                },
                Err(e) => return Err(e), // 如果读取文件失败，则返回错误
            }
        }
    }

    Ok((natives, texs))
}
fn test_totex(){
    let dir = "./src/tex"; // 使用当前目录，你可以改为任意目录路径
    let res = read_dir_files(Path::new(dir)); // 读取目录下所有文件
    match res {
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        },
        _ => {}
    }

    let (natives, texs) = res.unwrap();
    println!("{} files found, start testing", natives.len());
    
    let mut success = 0;
    let now = Instant::now();
    for i in 0..natives.len() {
        match ast_reader::read_ast(&natives[i]) {
            Ok (exp) => {
                // println!("Exp read successfully");
                // dbg!(exp);
                let tr = ast::tex_writer::TexWriter::new_exp(exp, HashMap::<String,bool>::new());
                let right_tex = texs[i].trim().to_string();
                let result = panic::catch_unwind(|| {
                    let tex = tr.to_tex().trim().to_string();
                    if tex != right_tex {
                        // println!("Tex not match:{}/{}", i, tex.len());
                        // println!("file: {}", natives[i]);
                        // println!("====================");
                        // println!("Expected: {}", texs[i]);
                        // println!("Actual: {}", tex);
                        // println!("====================");
                        panic!("Tex not match")
                    }
                });
                match result {
                    Ok(_) => {
                        success += 1;
                    },
                    Err(e) => {
                        // println!("file: {}", natives[i]);
                        println!("Error: {:?}", e);
                        return;
                    }
                }
                
            },
            Err(e) => {
                println!("read_ast error: {}/{}", i, natives.len());
                println!("====================");
                // println!("file: {}", natives[i]);
                println!("Parse error: {:?}", e);
                return;
            }
        }
    }
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
    println!("{}/{} files parsed successfully", success, natives.len());
}

fn main() -> std::io::Result<()> {
    test_read_tex();
    Ok(())
}