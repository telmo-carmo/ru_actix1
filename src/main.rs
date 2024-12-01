/*

cargo check

cargo build --release

cargo run -- 8000

*/

use std::env;
use chrono::{DateTime, Utc};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{Builder,Env};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{debug, info, warn};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Req1 {
    name: String,
    age: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    let start = SystemTime::now();
    let dt1: DateTime<Utc> = start.into();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs = since_the_epoch.as_secs();
    info!("Current time: {}, secs = {}",dt1.format("%Y-%m-%d %H:%M:%S").to_string(), secs);
    HttpResponse::Ok().body(format!("Hello world, {}",secs))
}

#[post("/echo")]
async fn echo(req: web::Json<Req1>) -> impl Responder {
    //warn!("Post /echo name={} age={}",req.name,req.age);
    warn!("Post /echo req={:?}",req);
    HttpResponse::Ok().body(format!("{};{}",req.name,req.age))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut port = 8080;
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    //env::set_var("RUST_LOG", "debug");
    //env_logger::init(); // Initialize the logger

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let number_str = &args[1];

        port = match number_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid port number: {}", number_str);
                8080
            }
        };
    }


    debug!("Running webserver on port {}",port );
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

