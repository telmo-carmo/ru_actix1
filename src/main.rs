/*

cargo check

cargo build --release

*/

//use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{Builder,Env};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{debug, info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
struct Req1 {
    name: String,
    age: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs = since_the_epoch.as_secs();
    info!("Current time: {}", secs);
    HttpResponse::Ok().body(format!("Hello world, {}",secs))
}

#[post("/echo")]
async fn echo(req: web::Json<Req1>) -> impl Responder {
    warn!("Post /echo {}",req);
    HttpResponse::Ok().body(req.name)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    //env::set_var("RUST_LOG", "debug");
    //env_logger::init(); // Initialize the logger

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

