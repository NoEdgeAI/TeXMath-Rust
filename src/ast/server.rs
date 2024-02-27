use axum::{
    routing::post,
    Router,
    extract::Json,
};
use std::net::SocketAddr;

use crate::ast;

pub async fn run_server(addr: String, port: u16) {
    let app = Router::new().route("/convert", post(convert_handler));
    println!("Listening on: {}:{}", addr, port);
    let addr = format!("{}:{}", addr, port).parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn convert_handler(Json(req): Json<ServerRequest>) -> Json<ServerResponse> {
    let mut envs = std::collections::HashMap::new();
    envs.insert("amsmath".to_string(), true);
    envs.insert("amssymb".to_string(), true);
    envs.insert("mathbb".to_string(), true);
    Json(native_to_tex(req.text.as_str(), &envs))
}
#[derive(serde::Deserialize)]
struct ServerRequest {
    from: String,
    to: String,
    text: String,
}

#[derive(serde::Serialize)]
struct ServerResponse {
    output: String,
    error: String,
}
fn native_to_tex(native: &str, envs: &std::collections::HashMap<String, bool>) -> ServerResponse{
    let ast = ast::ast_reader::read_ast(native);
    match ast {
        Ok(ast) => {
            let tex = ast::tex_writer::write_tex_with_md(ast, envs);
            match tex {
                Ok(tex) => {
                    ServerResponse {
                        output: tex,
                        error: "".to_string(),
                    }
                }
                Err(e) => {
                    ServerResponse {
                        output: "".to_string(),
                        error: "write_tex: ".to_string() + e.to_string().as_str(),
                    }
                }
            }
        }
        Err(e) => {
            ServerResponse {
                output: "".to_string(),
                error: "read_ast: ".to_string() + e.to_string().as_str(),
            }
        }
    }
}