

#[macro_use]
extern crate diesel;

/*

cargo install diesel_cli --no-default-features --features "sqlite-bundled"
diesel setup

diesel migration generate create_bonuses
diesel migration run

diesel print-schema

--
cargo check

cargo build --release

cargo run -- 8000

*/

use std::env;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
//use env_logger::{Builder, Env};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use rand::Rng;

mod schema;
mod models;
use schema::{bonus,dept};
use models::{Bonus,Dept};

mod jwt_mw;

#[derive(Deserialize,Debug)]
struct Req1 {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct Resp1 {
    id: u32,
    name: String,
}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


#[get("/")]
async fn index() -> impl Responder {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs = since_the_epoch.as_secs();
    let dt: DateTime<Utc> = DateTime::from(start);
    let dts = dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
    info!("Current time: {}, secs: {}",dts, secs);
    HttpResponse::Ok().body(format!("Hello world, {}",secs))
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
async fn create_bonus(pool: web::Data<DbPool>, pbonus: web::Json<Bonus>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let new_bonus = Bonus {
        ename: pbonus.ename.clone(),
        job: pbonus.job.clone(),
        sal: pbonus.sal,
        comm: pbonus.comm,
    };

    diesel::insert_into(bonus::table)
        .values(&new_bonus)
        .execute(&mut conn)
        .expect("Error saving new bonus");

    info!("created bonus name={}",new_bonus.ename);
    HttpResponse::Created().json(new_bonus)
}

#[get("/api/bonus")]
async fn get_all_bonus(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = bonus::table.load::<Bonus>(&mut conn).expect("Error loading bonuses");
    info!("bonus get all #={}",results.len());
    HttpResponse::Ok().json(results)
}


#[get("/api/bonus/{id}")]
async fn get_bonus(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    info!("bonus get id={}",id);
    let result = bonus::table.find(id.into_inner()).first::<Bonus>(&mut conn);

    match result {
        Ok(bonus) => HttpResponse::Ok().json(bonus),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}


#[put("/api/bonus/{id}")]
async fn update_bonus(pool: web::Data<DbPool>, id: web::Path<String>, pbonus: web::Json<Bonus>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    info!("bonus update id={}",id);
    let updated_bonus = Bonus {
        ename: pbonus.ename.clone(),
        job: pbonus.job.clone(),
        sal: pbonus.sal,
        comm: pbonus.comm,
    };

    match diesel::update(bonus::table.find(id.into_inner()))
        .set(&updated_bonus)
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json(updated_bonus),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[delete("/api/bonus/{id}")]
async fn delete_bonus(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    info!("bonus delete id={}",id);
    diesel::delete(bonus::table.find(id.into_inner()))
        .execute(&mut conn)
        .expect("Error deleting bonus");

    HttpResponse::Ok().finish()
}

#[get("/api/dept")]
async fn get_all_dept(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = dept::table.load::<Dept>(&mut conn).expect("Error loading departments");
    info!("depts get all #={}",results.len());
    HttpResponse::Ok().json(results)
}

#[get("/api/dept/{id}")]
async fn get_dept(pool: web::Data<DbPool>, deptno: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    info!("dept get deptno={}", deptno);
    let id = *deptno;
    let result = dept::table.find(id).first::<Dept>(&mut conn);

    match result {
        Ok(fdept) => HttpResponse::Ok().json(fdept),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/auth/rnd")]
async fn get_rnd() -> impl Responder {
    // Validate the JWT token

    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    warn!("rnd = {}", random_number);
    HttpResponse::Ok().json(random_number)
}

#[derive(Deserialize)]
struct LoginRequest {
    uid: String,
    pwd: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[post("/auth/login")]
async fn do_login(req: web::Json<LoginRequest>) -> impl Responder {
    let username = &req.uid;
    let password = &req.pwd;

    // Validate username and password (this is a placeholder, replace with actual validation)
    if username == "demo" && password == "123" {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(3600)) // Token expires in 1 hour
            .expect("Valid timestamp")
            .timestamp() as usize;
        let token = match jwt_mw::generate_jwt(&username,"NB",expiration) {
            Ok(s) => s,
            Err(e) => format!("failed to gen JWT: {}",e),
        };
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut port = 8080;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let s_port = &args[1];
        port = match s_port.parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error in port  argument: {}", e);
                8080
            }
        }
    }

    //Builder::from_env(Env::default().default_filter_or("debug")).init();
    //env::set_var("RUST_LOG", "debug");
    dotenv().ok();
    env_logger::init(); // Initialize the loggerq

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    debug!("Running webserver on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(index)
            .service(echo)
            .service(create_bonus)
            .service(get_all_bonus)
            .service(get_bonus)
            .service(update_bonus)
            .service(delete_bonus)
            .service(get_all_dept)
            .service(get_dept)
            .service(get_rnd)
            .service(do_login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
