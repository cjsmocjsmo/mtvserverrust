use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{Builder, Target};
use mpvipc::{Error, Mpv, MpvCommand, PlaylistAddOptions};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::Command;
use std::str::FromStr;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

pub mod envvars;
pub mod servermov;
pub mod servertvs;
pub mod serverutils;

#[actix_web::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("MTV Start");
    let _vars = envvars::set_env_vars();
    log::info!("Env Vars have been set");
    let _mpv = init_mpv();
    println!("MPV has been initialized");

    Builder::new().target(Target::Stdout).init();

    let thumb_path = env::var("MTV_THUMBNAIL_PATH").expect("MTV_THUMBNAIL_PATH not set");

    let socket = gen_server_addr();
    println!("go to: http://{}/", socket.clone());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(serverutils::hello)
            .service(serverutils::get_stats)
            .service(serverutils::run_setup_check)
            .service(servermov::action)
            .service(servermov::arnold)
            .service(servermov::brucelee)
            .service(servermov::brucewillis)
            .service(servermov::cartoons)
            .service(servermov::chucknorris)
            .service(servermov::comedy)
            .service(servermov::drama)
            .service(servermov::documentary)
            .service(servermov::fantasy)
            .service(servermov::godzilla)
            .service(servermov::harrypotter)
            .service(servermov::indianajones)
            .service(servermov::jamesbond)
            .service(servermov::johnwayne)
            .service(servermov::johnwick)
            .service(servermov::jurassicpark)
            .service(servermov::kingsmen)
            .service(servermov::meninblack)
            .service(servermov::misc)
            .service(servermov::nicolascage)
            .service(servermov::pirates)
            .service(servermov::riddick)
            .service(servermov::starwars)
            .service(servermov::startrek)
            .service(servermov::superheroes)
            .service(servermov::scifi)
            .service(servermov::tomcruize)
            .service(servermov::transformers)
            .service(servermov::tremors)
            .service(servermov::therock)
            .service(servermov::xmen)
            .service(servermov::buzz)
            .service(servermov::charliebrown)
            .service(servermov::minions)
            .service(servermov::oldies)
            .service(servermov::tinkerbell)
            .service(servermov::stalone)
            .service(servertvs::fuubar)
            .service(servertvs::houseofthedragon)
            .service(servertvs::ringsofpower)
            .service(servertvs::wheeloftime)
            .service(servertvs::voyager)
            .service(servertvs::sttv)
            .service(servertvs::enterprise)
            .service(servertvs::tng)
            .service(servertvs::discovery)
            .service(servertvs::picard)
            .service(servertvs::lowerdecks)
            .service(servertvs::prodigy)
            .service(servertvs::strangenewworlds)
            .service(servertvs::andor)
            .service(servertvs::badbatch)
            .service(servertvs::bobafett)
            .service(servertvs::obiwankenobi)
            .service(servertvs::mandalorian)
            .service(servertvs::talesofthejedi)
            .service(servertvs::visions)
            .service(servertvs::silo)
            .service(servertvs::thelastofus)
            .service(servertvs::foundation)
            .service(servertvs::alteredcarbon)
            .service(servertvs::cowboybebop)
            .service(servertvs::forallmankind)
            .service(servertvs::lostinspace)
            .service(servertvs::raisedbywolves)
            .service(servertvs::nightsky)
            .service(servertvs::orville)
            .service(servertvs::halo)
            .service(servertvs::secretinvasion)
            .service(servertvs::falconwintersoldier)
            .service(servertvs::hawkeye)
            .service(servertvs::iamgroot)
            .service(servertvs::loki)
            .service(servertvs::moonknight)
            .service(servertvs::msmarvel)
            .service(servertvs::shehulk)
            .service(servertvs::wandavision)
            .service(servertvs::hford1923)
            .service(servertvs::prehistoricplanet)
            .service(servertvs::ahsoka)
            .service(servertvs::thecontinental)
            .service(servertvs::monarchlegacyofmonsters)
            .service(servertvs::shogun)
            .service(servertvs::fallout)
            .service(servertvs::threebodyproblem)
            .service(startmov)
            .service(starttv)
            .service(pause)
            .service(resume)
            .service(stop)
            .service(fs::Files::new("/thumbnails", thumb_path.clone()).show_files_listing())
    })
    .bind(socket)?
    .run()
    .await?;

    Ok(())
}

pub fn init_mpv() {
    Command::new("mpv")
        .arg("--idle")
        .arg("--input-ipc-server=/tmp/mpvsocket")
        .spawn()
        .expect("Failed to start mpv");
}

pub fn gen_server_addr() -> SocketAddr {
    let raw_addr = env::var("MTV_RAW_ADDR").expect("MTV_RAW_ADDR not set");
    let mtv_v4_addr = Ipv4Addr::from_str(&raw_addr).unwrap();
    let port: u16 = env::var("MTV_SERVER_PORT")
        .expect("MTV_SERVER_PORT not set")
        .parse()
        .unwrap();
    let socket = SocketAddr::new(IpAddr::V4(mtv_v4_addr), port);

    socket
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub name: String,
    pub year: String,
    pub posteraddr: String,
    pub size: String,
    pub path: String,
    pub idx: String,
    pub movid: String,
    pub catagory: String,
    pub httpthumbpath: String,
}
#[get("/startmov/{mediaid}")]
pub async fn startmov(id: web::Path<String>) -> impl Responder {
    let mediaid = id.into_inner();
    println!("mediaid: {}", mediaid.clone());
    log::info!("mediaid: {}", mediaid.clone());
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM movies WHERE movid = ?1")
        .unwrap();

    let mut rows = stmt.query(&[&mediaid]).expect("Unable to query db");
    let mut result = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let movie = Movie {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            year: row.get(2).unwrap(),
            posteraddr: row.get(3).unwrap(),
            size: row.get(4).unwrap(),
            path: row.get(5).unwrap(),
            idx: row.get(6).unwrap(),
            movid: row.get(7).unwrap(),
            catagory: row.get(8).unwrap(),
            httpthumbpath: row.get(9).unwrap(),
        };
        result.push(movie);
    }
    println!("movpath: {:?}", result[0].path.clone());
    log::info!("movpath: {:?}", result[0].path.clone());

    let _ = start_media(result[0].path.clone());
    let result = format!("Playing: {}", result[0].path.clone());

    HttpResponse::Ok().body(result)
}

#[get("/starttv/{mediaid}")]
pub async fn starttv(id: web::Path<String>) -> impl Responder {
    let mediaid = id.into_inner();
    println!("Playing: {}", mediaid.clone());
    log::info!("Playing: {}", mediaid.clone());

    // let _ = start_media(mediapath.clone());
    let result = format!("Playing: {}", mediaid.clone());

    HttpResponse::Ok().body(result)
}

pub async fn start_media(media: String) -> Result<(), Error> {
    let socket_path = "/tmp/mpvsocket";
    let mpv = Mpv::connect(socket_path)?;
    mpv.set_property("fullscreen", true)?;
    mpv.run_command(MpvCommand::LoadFile {
        file: media.into(),
        option: PlaylistAddOptions::Replace
    })?;
    // mpv.disconnect();

    Ok(())
}

#[get("/pause")]
pub async fn pause() -> impl Responder {
    let _ = pause_media();

    HttpResponse::Ok().body("Paused")
}

pub fn pause_media() -> Result<(), Error> {
    let socket_path = "/tmp/mpvsocket";
    let mpv = Mpv::connect(socket_path)?;
    mpv.set_property("pause", true)?;
    mpv.disconnect();

    Ok(())
}

#[get("/resume")]
pub async fn resume() -> impl Responder {
    let _ = play_media();

    HttpResponse::Ok().body("Playing")
}

pub fn play_media() -> Result<(), Error> {
    let socket_path = "/tmp/mpvsocket";
    let mpv = Mpv::connect(socket_path)?;
    mpv.set_property("pause", false)?;
    mpv.disconnect();

    Ok(())
}

#[get("/stop")]
pub async fn stop() -> impl Responder {
    let _ = stop_media();

    HttpResponse::Ok().body("Stopped")
}

pub fn stop_media() -> Result<(), Error> {
    let socket_path = "/tmp/mpvsocket";
    let mpv = Mpv::connect(socket_path)?;
    mpv.run_command(MpvCommand::Stop)?;
    mpv.disconnect();

    Ok(())
}
