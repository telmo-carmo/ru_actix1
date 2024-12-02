/*

cargo check

cargo build --release

*/


//use std::env;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{Builder, Env};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize)]
struct Req1 {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct Resp1 {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bonus {
    pub ename: String,
    pub job: String,
    pub sal: i32,
    pub comm: i32,
}

type BonusStore = Mutex<HashMap<u32, Bonus>>;

#[get("/")]
async fn index() -> impl Responder {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let secs = since_the_epoch.as_secs();
    info!("Current time: {}", secs);
    HttpResponse::Ok().body(format!("Hello world, {}", secs))
}

#[post("/echo")]
async fn echo(req: web::Json<Req1>) -> impl Responder {
    warn!("Post /echo name={} age={}", req.name, req.age);
    let rs = Resp1 {
        id: 1234566,
        name: req.name.clone(),
    };
    HttpResponse::Ok().json(rs)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Bonus CRUD
#[post("/api/bonus")]
async fn create_bonus(
    bonus: web::Json<Bonus>,
    bonus_store: web::Data<BonusStore>,
) -> impl Responder {
    let mut store = bonus_store.lock().unwrap();
    let id = store.len() as u32 + 1;
    store.insert(id, bonus.into_inner());
    info!("created bonus id={}",id);
    HttpResponse::Created().json(id)
}

#[get("/api/bonus")]
async fn get_all_bonus(bonus_store: web::Data<BonusStore>) -> impl Responder {
    let store = bonus_store.lock().unwrap();
    let bonuses: Vec<Bonus> = store.values().cloned().collect();
    info!("get all bonus #={}", bonuses.len());
    HttpResponse::Ok().json(bonuses)
}

#[get("/api/bonus/{id}")]
async fn get_bonus(id: web::Path<u32>, bonus_store: web::Data<BonusStore>) -> impl Responder {
    let store = bonus_store.lock().unwrap();
    info!("get bonus id={}",id);
    if let Some(bonus) = store.get(&id) {
        HttpResponse::Ok().json(bonus)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[put("/api/bonus/{id}")]
async fn update_bonus( id: web::Path<u32>,bonus: web::Json<Bonus>, bonus_store: web::Data<BonusStore>) -> impl Responder {
    let mut store = bonus_store.lock().unwrap();
    if store.contains_key(&id) {
        let item_id: u32 = *id;
        store.insert(item_id, bonus.into_inner());
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/api/bonus/{id}")]
async fn delete_bonus(id: web::Path<u32>, bonus_store: web::Data<BonusStore>) -> impl Responder {
    let mut store = bonus_store.lock().unwrap();
    info!("delete bonus id={}",id);
    if store.remove(&id).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    //env::set_var("RUST_LOG", "debug");
    //env_logger::init(); // Initialize the logger

    debug!("Running webserver on port {}", port);
    let bonus_store = web::Data::new(Mutex::new(HashMap::<u32,Bonus>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(bonus_store.clone())
            .service(index)
            .service(echo)
            .service(create_bonus)
            .service(get_all_bonus)
            .service(get_bonus)
            .service(update_bonus)
            .service(delete_bonus)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
