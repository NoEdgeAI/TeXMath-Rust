use std::collections::HashMap;
use std::{fs, panic};
use std::path::Path;
use std::io;
use std::io::Write;

mod ast;
use std::time::Instant;
use ast::judge::{judge_by_texmath, JudgeResult};

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

        // 跳过不是.test文件
        if path.extension().unwrap() != "test" {
            continue;
        }
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

fn pretty_print_hex(output: String) -> String{
    // 第一行显示hex, 第二行显示字符:
    // 40 41 42
    // @  A  B

    // 把0D 0A替换成0A

    let output = output.replace("\r\n", "\n");
    let mut hex = String::new();
    let mut cs = String::new();
    for c in output.chars() {
        hex.push_str(&format!("{:02x} ", c as u8));
        match c {
            '\n' => cs.push_str("\\n "),
            '\t' => cs.push_str("\\t "),
            '\r' => cs.push_str("\\r "),
            _ => cs.push_str(&format!("{}  ", c)),
        }
    }
    return format!("{}\n{}", hex, cs);
}
fn test_totex_and_judge(){
    let dir = "./src/failed";

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
    envs.insert("mathbb".to_string(), true);
    for i in 0..natives.len() {
        match ast_reader::read_ast(&natives[i]) {
            Ok (exp) => {
                println!("===============================");
                println!("Exp read successfully: {}/{}", i+1, natives.len());
                println!("Filename: {}", filenames[i]);
                // std::thread::sleep(std::time::Duration::from_millis(100));
                // dbg!(exp);
                let totex_res = ast::tex_writer::write_tex_with_env(exp, &envs);
                match totex_res{
                    Ok(tex) => {
                        // println!("Exp to tex successfully");
                        // dbg!(tex);

                        let right_tex = texs[i].trim().to_string();
                        let native = natives[i].trim().to_string();
                        let (jr, texmath_res) = judge_by_texmath(right_tex.clone(), tex.clone());
                        // if jr != JudgeResult::Same{
                        //     panic!("to_test error: {}/{}: {file}", i+1, natives.len(), file = filenames[i]);
                        // }
                        let same = jr == JudgeResult::Same || jr == JudgeResult::Equivalent;
                        if same || pretty_print_hex(right_tex.clone()) == pretty_print_hex(tex.clone()) {
                            // println!("to_test ok: {}/{}", i+1, natives.len());
                            let right_tex = texs[i].trim().to_string().replace("\r\n", "\n");
                            let tex = tex.trim().to_string().replace("\r\n", "\n");
                            if right_tex == tex {
                                println!("Judge: {} : {}/{}", jr.to_str(),i+1, natives.len());
                            } else {
                                println!("Judge: {} : {}/{}", jr.to_str(), i+1, natives.len());
                            }
                            success += 1;
                            continue;
                        }
                        // write to file
                        let f = fs::File::create("./output").unwrap();
                        let mut f = io::BufWriter::new(f);
                        println!("same: {} = {:?}", same, jr.to_str());

                        f.write("filename:".as_bytes()).unwrap();
                        f.write(filenames[i].as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();
                        f.write("same:".as_bytes()).unwrap();
                        f.write(same.to_string().as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();

                        f.write("ast:\n".as_bytes()).unwrap();
                        f.write(native.as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();

                        f.write("tex:\n".as_bytes()).unwrap();
                        f.write(tex.as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();

                        f.write("expect:\n".as_bytes()).unwrap();
                        f.write(right_tex.as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();

                        f.write("texmath:\n".as_bytes()).unwrap();
                        f.write(texmath_res.as_bytes()).unwrap();
                        f.write("\n\n".as_bytes()).unwrap();

                        // bytes hex:
                        f.write(pretty_print_hex(right_tex.clone()).as_bytes()).unwrap();
                        f.write("\n".as_bytes()).unwrap();

                        f.write(pretty_print_hex(tex.clone()).as_bytes()).unwrap();
                        f.write("\n".as_bytes()).unwrap();
                        // panic!("to_test error: {}/{}: {file}", i+1, natives.len(), file = filenames[i]);
                        println!("to_test error: {}/{}: {file}", i+1, natives.len(), file = filenames[i]);
                    },
                    Err(e) => {
                        println!("Exp to tex error: {}/{}", i, natives.len());
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

    println!("===============================");
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
    println!("{}/{} files parsed successfully", success, natives.len());
    println!("rate: {}%", success as f64 / natives.len() as f64 * 100.0);
}

fn bench_test_totex(){
    let now = Instant::now();
    let dir = "./src/test";

    let res = read_dir_files(Path::new(dir)); // 读取目录下所有文件
    match res {
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        },
        _ => {}
    }

    let (filenames, natives, texs) = res.unwrap();

    println!("{} files found, start testing, using {} ms", natives.len(), now.elapsed().as_millis());

    let mut success = 0;
    let now = Instant::now();
    let mut envs = HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    envs.insert("mathbb".to_string(), true);
    for i in 0..natives.len() {
        match ast_reader::read_ast(&natives[i]) {
            Ok (exp) => {
                let totex_res = ast::tex_writer::write_tex_with_env(exp, &envs);
                match totex_res{
                    Ok(_) => {
                        success += 1;
                    },
                    Err(e) => {
                        println!("Exp to tex error: {}/{}", i, natives.len());
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

    println!("===============================");
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
    println!("{}/{} files parsed successfully", success, natives.len());
    println!("rate: {}%", success as f64 / natives.len() as f64 * 100.0);
}
fn main() -> io::Result<()> {
    test_totex_and_judge();
    Ok(())
}