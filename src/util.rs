use std::{fs::OpenOptions, io::{Read, Write}};
use axum::{extract::Path, http::{HeaderMap, StatusCode, header}, response::IntoResponse, Json};
use tokio::{fs::File, io::AsyncReadExt};
use tower::ServiceBuilder;
use tower_http::{classify::{ServerErrorsAsFailures, SharedClassifier}, services::ServeDir, trace::{Trace, TraceLayer}};

use crate::directory::Directory;

pub struct AppState {}

pub fn traced_service(path: &str) -> Trace<ServeDir, SharedClassifier<ServerErrorsAsFailures>> {
    ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .service(ServeDir::new(format!("data/{path}")))
}

pub async fn write_archives() {
    let mut file = OpenOptions::new()
        .truncate(true)
        .create(false)
        .write(true)
        .open(format!("data/Archives/data.md")).unwrap();

    let buf = ">>REDEMPTION$d\n>>>README.txt\n>>>key.dll$k:19931,H0PE";
    file.write(buf.as_bytes()).unwrap();

    for i in 1..100 {
        let buf = if i == 19 { format!("\n>>>{i}$d\n>>>>arguments.txt") } else { format!("\n>>>{i}$d") };
        file.write(buf.as_bytes()).unwrap();
    }
}

pub async fn download(Path(ip): Path<String>) -> StatusCode {
    let mut roots = if let Ok(file) = OpenOptions::new()
        .read(true)
        .append(true)
        .create(false)
        .write(true)
        .open(format!("data/Home/roots.txt")) {file} else { return StatusCode::INTERNAL_SERVER_ERROR };

    let mut buffer = String::new();
    if let Err(_) = roots.read_to_string(&mut buffer) { return StatusCode::INTERNAL_SERVER_ERROR };

    if buffer.contains(ip.to_string().as_str()) { return StatusCode::BAD_REQUEST };
    let mut content = format!("\n{ip}");
    if let Err(_) = roots.write(content.as_bytes()) { return StatusCode::INTERNAL_SERVER_ERROR } ;
    content.clear();

    println!("hello");
    let mut data = if let Ok(file) = OpenOptions::new()
        .append(true)
        .create(false)
        .write(true)
        .open(format!("data/Home/data.md")) {file} else { return StatusCode::INTERNAL_SERVER_ERROR };

    content.push('\n');
    let file_data = if let Ok(content) = std::fs::read_to_string(format!("data/{ip}/data.md")) { content } else { return StatusCode::BAD_REQUEST };
    content.push_str(&file_data);
    if let Err(_) = data.write(content.as_bytes()) { return StatusCode::INTERNAL_SERVER_ERROR };
    
    StatusCode::OK
}

pub async fn reset() {
    let mut roots = OpenOptions::new()
        .append(false)
        .truncate(true)
        .create(false)
        .write(true)
        .open(format!("data/Home/roots.txt")).unwrap();

    roots.write("Home".as_bytes()).unwrap();

    let mut data = OpenOptions::new()
        .append(false)
        .truncate(true)
        .create(false)
        .write(true)
        .open(format!("data/Home/data.md")).unwrap();

    let content = std::fs::read_to_string("data/Home/data_og.md").unwrap();
    data.write(content.as_bytes()).unwrap();
}

pub async fn read() -> Json<Directory> {
    let file = std::fs::read_to_string("../home.md").unwrap();
    Json(Directory::parse_file(file))
}

pub async fn get_file(Path((computer, file)): Path<(String, String)>) -> impl IntoResponse {
    let path = format!("./data/{computer}/{}", get_file_true_path(file));
    println!("Asking for file {path}");

    match File::open(&path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer).await { return StatusCode::INTERNAL_SERVER_ERROR.into_response()};
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream();
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, mime_type.as_ref().parse().unwrap());

            (headers, buffer).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}   

fn get_file_true_path(file: String) -> String {
    let mut it = file.split(".");
    let mut og = it.next().unwrap().to_string();
    if let Some(ext) = it.next() { 
        if ext == "ps" { og.push_str(".txt"); }
        else { og.push('.'); og.push_str(ext); }
    }
    else { og.push_str(".txt");}

    og
}