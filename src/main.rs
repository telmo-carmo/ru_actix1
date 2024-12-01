
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/")]
async fn index() -> impl Responder {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let secs =  = since_the_epoch.as_secs();
    HttpResponse::Ok().body(format!("Hello world, {}",secs))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

