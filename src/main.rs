use std::collections::HashMap;
use std::{fs, panic};
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

// filename, native, tex
fn read_dir_files(dir: &Path) -> io::Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut filenames = Vec::new();
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

                            // 文件名
                            filenames.push(path.file_name().unwrap().to_str().unwrap().to_string());
                        }
                    }
                },
                Err(e) => return Err(e), // 如果读取文件失败，则返回错误
            }
        }
    }

    Ok((filenames, natives, texs))
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

    let (filenames, natives, texs) = res.unwrap();
    
    println!("{} files found, start testing", natives.len());
    
    let mut success = 0;
    let now = Instant::now();
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    for i in 0..natives.len() {
        match ast_reader::read_ast(&natives[i]) {
            Ok (exp) => {
                println!("Exp read successfully");
                // dbg!(exp);
                let tex = ast::tex_writer::write_tex_with_env(exp, &envs);
                let right_tex = texs[i].trim().to_string();
                let result = panic::catch_unwind(|| {
                    println!("===============================");
                    println!("filename=======================");
                    println!("\n{}\n", filenames[i]);
                    println!("===============================");
                    println!("native=========================");
                    println!("\n{}\n", natives[i]);
                    println!("===============================");
                    println!("tex============================");
                    println!("\n{:?}\n", tex);
                    println!("right_tex======================");
                    println!("\n{}\n", right_tex);
                    let tex = tex.unwrap();
                    assert_eq!(tex, right_tex);
                });
                match result {
                    Ok(_) => {
                        success += 1;
                    },
                    Err(e) => {
                        // panic: assertion failed
                        println!("===============================");
                        println!("filename=======================");
                        println!("\n{}\n", filenames[i]);
                        println!("===============================");
                        println!("native=========================");
                        println!("\n{}\n", natives[i]);
                        println!("===============================");
                        println!("tex============================");
                        println!("\n{}\n", texs[i]);
                        println!("err============================");
                        dbg!("{:?}", e);
                        return;
                    }
                }
                
            },
            Err(e) => {
                // read_ast error
                println!("read_ast error: {}/{}", i, natives.len());
                println!("===============================");
                println!("filename=======================");
                println!("\n{}\n", filenames[i]);
                println!("===============================");
                println!("native=========================");
                println!("\n{}\n", natives[i]);
                println!("===============================");
                println!("tex============================");
                println!("\n{}\n", texs[i]);
                println!("===============================");
                println!("Parse error: {:?}", e);
                println!("===============================");
                return;
            }
        }
    }
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
    println!("{}/{} files parsed successfully", success, natives.len());
}

fn main() -> io::Result<()> {
    test_totex();
    Ok(())
}