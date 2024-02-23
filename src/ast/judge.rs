use std::io::Write;
use serde_json::json;

#[test]
fn test_judge_by_mathjax(){
    let test_tex = r#"
f(x) = \begin{cases}
1 &  - 1 \le x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let right_tex = r#"
f(x) = \begin{cases}
1 & - 1 \leq x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let res = judge_by_mathjax(right_tex.to_string(), test_tex.to_string());
    println!("res: {:?}", res);
}

const MATHJAX_URL: &str = "http://nps.noedgeai.com:10088/generateSvgPng";
pub fn judge_by_mathjax(right_tex: String, test_tex: String) -> bool{
    let test_tex_json = json!(
        {
            "data": test_tex,
        }
    );
    let right_tex_json = json!(
        {
            "data": right_tex,
        }
    );

    let client = reqwest::blocking::Client::new();

    let res = client.post(MATHJAX_URL)
        .json(&test_tex_json).send().unwrap();

    if res.status() != 200{
        println!("status: {}", res.status());
        println!("headers: {:#?}", res.headers());
        println!("json: {:#?}", test_tex_json);
        return false
    }

    let test_tex_png = res.bytes().unwrap();

    let res = client.post(MATHJAX_URL)
        .json(&right_tex_json).send().unwrap();
    if res.status() != 200{
        println!("status: {}", res.status());
        println!("headers: {:#?}", res.headers());
        println!("json: {:#?}", right_tex_json);
        return false
    }

    let right_tex_png = res.bytes().unwrap();
    println!("A: {:?}", test_tex_png.as_ref());
    println!("B: {:?}", right_tex_png.as_ref());
    // write to file: ./test.png
    let mut file = std::fs::File::create("./test.png").unwrap();
    file.write(test_tex_png.as_ref()).unwrap();

    let mut file = std::fs::File::create("./right.png").unwrap();
    file.write(right_tex_png.as_ref()).unwrap();
    return test_tex_png == right_tex_png;
}

#[test]
fn test_judge_by_texmath(){
    let test_tex = r#"
f(x) = \begin{cases}
1 &  - 1 \le x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let right_tex = r#"
f(x) = \begin{cases}
1 & - 1 \leq x < 0 \\
\frac{1}{2} & x = 0 \\
1 - x^{2} & \text{otherwise}
\end{cases}
    "#;
    let (flag, res) = judge_by_texmath(right_tex.to_string(), test_tex.to_string());
    println!("res: \n{}\nflag: {:?}", res, flag);
}

#[derive(Debug, PartialEq)]
pub enum JudgeResult{
    Same,
    Equivalent, // 语义等价
    Different,
    Error,
}

impl JudgeResult {
    pub fn to_str(&self) -> &str {
        match self {
            JudgeResult::Same => "same",
            JudgeResult::Equivalent => "equivalent",
            JudgeResult::Different => "different",
            JudgeResult::Error => "error",
        }
    }
}

pub fn judge_by_texmath(right_tex: String, test_tex: String) -> (JudgeResult, String){
    let right_tex = right_tex.trim().to_string().replace("\r\n", "\n");
    let test_tex = test_tex.trim().to_string().replace("\r\n", "\n");
    if right_tex == test_tex{
        return (JudgeResult::Same, test_tex);
    }


    let test_tex_json = json!(
        {
            "display": false,
            "from": "tex",
            "to": "tex",
            "text": test_tex
        }
    );
    let right_tex_json = json!(
        {
            "display": false,
            "from": "tex",
            "to": "tex",
            "text": right_tex
        }
    );


    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:30000/convert")
        .json(&test_tex_json).send().unwrap();
    // println!("status: {}", res.status());
    // println!("headers: {:#?}", res.headers());
    if res.status() != 200{
        println!("status: {}", res.status());
        println!("headers: {:#?}", res.headers());
        println!("json: {:#?}", test_tex_json);
        return (JudgeResult::Error, "".to_string())
    }
    let test_tex = res.text().unwrap().trim().to_string().replace("\r\n", "\n");

    let res = client.post("http://localhost:30000/convert")
        .json(&right_tex_json).send().unwrap();
    if res.status() != 200{
        println!("status: {}", res.status());
        println!("headers: {:#?}", res.headers());
        println!("json: {:#?}", right_tex_json);
        return (JudgeResult::Error, "".to_string())
    }

    let right_tex = res.text().unwrap().trim().to_string().replace("\r\n", "\n");
    // println!("A: {:?}", body.as_bytes());
    // println!("B: {:?}", right_tex.as_bytes());
    if test_tex == right_tex{
        return (JudgeResult::Equivalent, test_tex);
    }
    return (JudgeResult::Different, test_tex);
}
