use std::{fs::OpenOptions, io::Write, net::Ipv4Addr};
use axum::{extract::Path, http::{HeaderMap, StatusCode, header}, response::IntoResponse, Json};
use tokio::{fs::File, io::AsyncReadExt};

use crate::directory::Directory;

pub struct AppState {}


pub async fn download(Path(ip): Path<Ipv4Addr>) {
    let mut file = if let Ok(file) = OpenOptions::new()
        .append(true)
        .create(false)
        .write(true)
        .open(format!("data/{ip}/data.txt")) {file} else { return println!("wrong path: {}", ip) };
    
    if let Err(e) = writeln!(file, "hello world") { println!("{}", e.to_string()) };
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