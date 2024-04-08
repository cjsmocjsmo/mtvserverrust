use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/stats")]
pub async fn get_stats() -> impl Responder {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn.prepare("SELECT * FROM stats").unwrap();
    let mut rows = stmt.query([]).expect("Unable to query db");
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let stat = Stats {
            id: row.get(0).expect("Unable to get id"),
            moviecount: row.get(1).expect("Unable to get moviecount"),
            tvshowcount: row.get(2).expect("Unable to get tvshowcount"),
            postercount: row.get(3).expect("Unable to get postercount"),
            size: row.get(4).expect("Unable to get fsize"),
        };
        result.push(stat);
    }
    let results = serde_json::to_string(&result).unwrap();

    HttpResponse::Ok().body(results)
}

pub fn db_file_exists() -> bool {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let path = Path::new(&db_path);
    if path.exists() {
        return true;
    } else {
        return false;
    }
}

pub fn thumbnail_dir_exists() -> bool {
    let mtv_thumbnail_path =
        env::var("MTV_THUMBNAIL_PATH").expect("$MTV_THUMBNAIL_PATH is not set");
    let path = std::path::Path::new(&mtv_thumbnail_path);
    if path.exists() && path.is_dir() {
        return true
    } else {
        return false
    }
}

pub fn mtvsetup_file_check() -> bool {
    let save_addr = env::var("MTV_FILE_PATH").expect("MTV_FILE_PATH not set");
    let path = Path::new(&save_addr);

    path.exists()
}

#[get("/setupcheck/{status}")]
pub async fn run_setup_check(path: web::Path<String>) -> impl Responder {
    let _status = path.into_inner();
    let file_exists = db_file_exists().to_string();
    let dir_exists = thumbnail_dir_exists().to_string();
    let mtv_file = mtvsetup_file_check().to_string();
    let mut resp = "false".to_string();
    if dir_exists == "true" && file_exists == "true" && mtv_file == "true" {
        resp = "true".to_string();
    };
    HttpResponse::Ok().body(resp)
}

// #[get("/setup/{status}")]
// pub async fn run_setup(path: web::Path<String>) -> impl Responder {
//     let _fuck = path.into_inner();
//     let setup = setup::setup().to_string();
//     let file_exists = setup::mtv_tables::db_file_exists().to_string();
//     let dir_exists = setup::mtv_image::thumbnail_dir_exists().to_string();
//     let mtv_file = setup::mtv_utils::mtvsetup_file_check().to_string();
//     let mut resp = "false".to_string();
//     if setup == "true" && dir_exists == "true" && file_exists == "true" && mtv_file == "true" {
//         resp = "true".to_string();
//     };
//     HttpResponse::Ok().body(resp)
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieImage {
    pub id: u32,
    pub imgid: String,
    pub path: String,
    pub imgpath: String,
    pub size: String,
    pub name: String,
    pub thumbpath: String,
    pub idx: u32,
    pub httpthumbpath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub id: u32,
    pub moviecount: String,
    pub tvshowcount: String,
    pub postercount: String,
    pub size: String,
}