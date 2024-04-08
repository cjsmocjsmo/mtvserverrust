use actix_web::{get, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;
use std::process::Command;

#[get("/start/{media}")]
pub async fn start(media: String) -> impl Responder {
    let media = media.into_inner();
    let result = Command::new("mpv")
        .arg(media)
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(result)
}


#[get("/play")]
pub async fn play() -> impl Responder {
    let output = Command::new("mpc")
        .arg("toggle")
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(output.stdout)
}

#[get("/pause")]
pub async fn pause() -> impl Responder {
    let result = Command::new("mpc")
        .arg("toggle")
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(result)
}

#[get("/stop")]
pub async fn stop() -> impl Responder {
    let result = Command::new("mpc")
        .arg("stop")
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(result)
}

#[get("/next")]
pub async fn next() -> impl Responder {
    let result = Command::new("mpc")
        .arg("next")
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(result)
}

#[get("/previous")]
pub async fn previous() -> impl Responder {
    result = Command::new("mpc")
        .arg("prev")
        .output()
        .expect("failed to execute process");
    HttpResponse::Ok().body(result)
}