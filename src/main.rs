

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
use actix_web::{delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware};
use actix_files::Files;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use utoipa_swagger_ui::SwaggerUi;
use utoipa::{OpenApi};

use dotenv::dotenv;
//use env_logger::{Builder, Env};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use sanitize_filename;
use rand::Rng;

mod schema;
mod models;
mod jwt_mw;


use schema::{bonus,dept};
use models::{Bonus,Dept};


#[derive(Deserialize,Debug)]
struct Req1 {
    name: String,
    age: i32,
    born: String,
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
    HttpResponse::Ok().body(format!("Hello RU_actix1 {secs} - {dts}"))
}

#[post("/echo")]
async fn echo(req: web::Json<Req1>) -> impl Responder {
    warn!("Post /echo name={} age={}", req.name, req.age);
    let mut rng = rand::thread_rng();
    let random_id: u32 = rng.gen_range(0..1000);
    let rs = Resp1 {
        id: random_id,
        name: req.name.clone(),
    };
    HttpResponse::Ok().json(rs)
}

async fn manual_hello() -> impl Responder { 
    let dt = chrono::Local::now();
    let dts = dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
    HttpResponse::Ok().body(format!("Hey there at {dts}"))
}

// Bonus CRUD

#[utoipa::path(
    responses(
        (status = 200, description = "Create one bonus in database")
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "Get all bonus from database")
    )
)]
#[get("/api/bonus")]
async fn get_all_bonus(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = bonus::table.load::<Bonus>(&mut conn).expect("Error loading bonuses");
    info!("bonus get all #={}",results.len());
    HttpResponse::Ok().json(results)
}


#[utoipa::path(
    responses(
        (status = 200, description = "Get one bonus from database")
    )
)]
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


#[utoipa::path(
    responses(
        (status = 200, description = "Update one bonus in database")
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "Delete one bonus in database")
    )
)]
#[delete("/api/bonus/{id}")]
async fn delete_bonus(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    info!("bonus delete id={}",id);
    diesel::delete(bonus::table.find(id.into_inner()))
        .execute(&mut conn)
        .expect("Error deleting bonus");

    HttpResponse::Ok().finish()
}


#[utoipa::path()]
#[get("/api/dept")]
async fn get_all_dept(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    let results = dept::table.load::<Dept>(&mut conn).expect("Error loading departments");
    info!("depts get all #={}",results.len());
    HttpResponse::Ok().json(results)
}

#[utoipa::path()]
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

#[utoipa::path()]
#[delete("/api/dept/{dept_no}")]
async fn delete_dept(pool: web::Data<DbPool>, dept_no: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    info!("delete Dept deptno={}", dept_no);
    diesel::delete(dept::table.filter(dept::columns::deptno.eq(dept_no.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting dept");

    HttpResponse::Ok().finish()
}

#[get("/auth/rnd")]
async fn get_rnd(req: HttpRequest) -> impl Responder {
    // Validate the JWT token
    let token = req.headers().get("Authorization").and_then(|header| header.to_str().ok());

    if let Some(token) = token {
        match jwt_mw::validate_jwt(token) {
            Ok(claims) => {
                // Token is valid, proceed
                let mut rng = rand::thread_rng();
                let random_number : f32 = rng.gen();
                warn!("rnd = {}, JWT user={}; role={}", random_number, claims.sub,claims.role);
                HttpResponse::Ok().json(random_number)
            }
            Err(_) => {
                return HttpResponse::Unauthorized().finish();
            }
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    }

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
        // get user role from db
        let user_role = "user";
        let token = match jwt_mw::generate_jwt(&username,user_role,expiration) {
            Ok(s) => s,
            Err(e) => format!("failed to gen JWT: {}",e),
        };
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}


#[post("/upload")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
 
    let mut rs  = "NONE Uploaded!".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));

        let filepath_cl = filepath.clone();
        let mut f = web::block(move|| std::fs::File::create(&filepath_cl))
            .await
            .unwrap()
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap().unwrap();
        }
        info!("File uploaded to {filepath}");
        rs = format!("File {filepath} uploaded successfully");
    }
    
    Ok(HttpResponse::Ok().body(rs))
}

#[post("/form1")]
async fn form1(form: web::Form<Req1>) -> impl Responder {
    info!("Form submitted with name={} age={} born={}", form.name, form.age,form.born);
    HttpResponse::Ok().body(format!("Received: name={}, age={}, born={}", form.name, form.age,form.born))
}


#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_bonus, get_bonus, create_bonus, update_bonus, delete_bonus,
        get_all_dept, get_dept, delete_dept,
    ),
    components(schemas(Bonus,Dept)),
    tags(
        (name = "Ru_actix1", description = "Ru-actix1 API Documentation")
    )
)]
struct ApiDoc;


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

    debug!("Running webserver on http://localhost:/{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(echo)
            .service(create_bonus)
            .service(get_all_bonus)
            .service(get_bonus)
            .service(update_bonus)
            .service(delete_bonus)
            .service(get_all_dept)
            .service(get_dept)
            .service(delete_dept)
            .service(get_rnd)
            .service(do_login)
            .service(form1)
            .service(upload_file)
            .service(Files::new("/static", "./static")) // Serve files from the "static"
            .service(SwaggerUi::new("/docs/{_:.*}")
                        .url("/openapi.json",  ApiDoc::openapi())) // Serve Swagger UI and /openapi.json
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?   // .bind(("0.0.0.0", port))?
    .run()
    .await
}
