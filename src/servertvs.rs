use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;
use serde::{Deserialize, Serialize};

#[get("/comedy/fuubar/{season}")]
pub async fn fuubar(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("FuuBar");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/houseofthedragon/{season}")]
pub async fn houseofthedragon(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("HouseOfTheDragon");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/ringsofpower/{season}")]
pub async fn ringsofpower(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("RingsOfPower");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/fantasy/wheeloftime/{season}")]
pub async fn wheeloftime(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("WheelOfTime");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/voyager/{season}")]
pub async fn voyager(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Voyager");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/sttv/{season}")]
pub async fn sttv(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("STTV");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/enterprise/{season}")]
pub async fn enterprise(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Enterprise");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/tng/{season}")]
pub async fn tng(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("TNG");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/discovery/{season}")]
pub async fn discovery(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Discovery");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/picard/{season}")]
pub async fn picard(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Picard");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/lowerdecks/{season}")]
pub async fn lowerdecks(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("LowerDecks");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/prodigy/{season}")]
pub async fn prodigy(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("Prodigy");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/startrek/strangenewworlds/{season}")]
pub async fn strangenewworlds(path: web::Path<String>) -> impl Responder {
    let season = path.into_inner();
    let catagory = String::from("StrangeNewWorlds");
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/ahsoka/{season}")]
pub async fn ahsoka(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Ahsoka");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/andor/{season}")]
pub async fn andor(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Andor");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/badbatch/{season}")]
pub async fn badbatch(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("TheBadBatch");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/bobafett/{season}")]
pub async fn bobafett(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("BobaFett");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/mandalorian/{season}")]
pub async fn mandalorian(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Mandalorian");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/talesofthejedi/{season}")]
pub async fn talesofthejedi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("TalesOfTheJedi");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/obiwan/{season}")]
pub async fn obiwankenobi(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("ObiWan");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/starwars/visions/{season}")]
pub async fn visions(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Visions");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/silo/{season}")]
pub async fn silo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Silo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/thelastofus/{season}")]
pub async fn thelastofus(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("TheLastOfUs");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/foundation/{season}")]
pub async fn foundation(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Foundation");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/alteredcarbon/{season}")]
pub async fn alteredcarbon(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("AlteredCarbon");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/cowboybebop/{season}")]
pub async fn cowboybebop(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("CowboyBebop");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/forallmankind/{season}")]
pub async fn forallmankind(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("ForAllManKind");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/lostinspace/{season}")]
pub async fn lostinspace(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("LostInSpace");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/raisedbywolves/{season}")]
pub async fn raisedbywolves(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("RaisedByWolves");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/nightsky/{season}")]
pub async fn nightsky(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("NightSky");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/orville/{season}")]
pub async fn orville(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Orville");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/halo/{season}")]
pub async fn halo(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Halo");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/prehistoricplanet/{season}")]
pub async fn prehistoricplanet(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("PrehistoricPlanet");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/secretinvasion/{season}")]
pub async fn secretinvasion(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("SecretInvasion");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/loki/{season}")]
pub async fn loki(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Loki");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/iamgroot/{season}")]
pub async fn iamgroot(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("IAmGroot");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/falconwintersoldier/{season}")]
pub async fn falconwintersoldier(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("FalconWinterSoldier");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/msmarvel/{season}")]
pub async fn msmarvel(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("MSMarvel");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/wandavision/{season}")]
pub async fn wandavision(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("WandaVision");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/moonknight/{season}")]
pub async fn moonknight(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("MoonKnight");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/hawkeye/{season}")]
pub async fn hawkeye(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("Hawkeye");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/shehulk/{season}")]
pub async fn shehulk(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("SheHulk");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/hford1923/{season}")]
pub async fn hford1923(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("HFord1923");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/thecontinental/{season}")]
pub async fn thecontinental(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("TheContinental");
    let season = path.into_inner();
    let result = get_shows(catagory, season).await;
    HttpResponse::Ok().body(result)
}

#[get("/scifi/monarchlegacyofmonsters/{season}")]
pub async fn monarchlegacyofmonsters(path: web::Path<String>) -> impl Responder {
    let catagory = String::from("MonarchLegacyOfMonsters");
    println!("catagory: {}", catagory);
    let season = path.into_inner();
    println!("season: {}", season);
    let result = get_shows(catagory, season).await;
    print!("result: {}", result);
    HttpResponse::Ok().body(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: u32,
    pub tvid: String,
    pub size: String,
    pub catagory: String,
    pub name: String,
    pub season: String,
    pub episode: String,
    pub path: String,
    pub idx: String,

}

async fn get_shows(cat: String, sea: String) -> String {
    let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM tvshows WHERE catagory = ?1 AND season = ?2 ORDER BY episode ASC")
        .unwrap();
    let mut rows = stmt.query(&[&cat, &sea]).expect("Unable to query db");
    let mut result = Vec::new();
    while let Some(row) = rows.next().expect("Unable to get next row") {
        let show = TVShow {
            id: row.get(0).expect("Unable to get id"),
            tvid: row.get(1).expect("Unable to get tvid"),
            size: row.get(2).expect("Unable to get size"),
            catagory: row.get(3).expect("Unable to get catagory"),
            name: row.get(4).expect("Unable to get name"),
            season: row.get(5).expect("Unable to get season"),
            episode: row.get(6).expect("Unable to get episode"),
            path: row.get(7).expect("Unable to get path"),
            idx: row.get(8).expect("Unable to get idx"),
        };
        result.push(show);
    }
    let result = serde_json::to_string(&result).unwrap();

    result
}
