use actix_web::{get, web, App, HttpResponse, Responder, HttpServer};
use actix_files as fs;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

use std::env;

async fn index() -> Result<fs::NamedFile, std::io::Error> {
    // serve the index.html from dist directory
    fs::NamedFile::open("../frontend/dist/index.html")
}

/// Handler to get a specific post by id
// #[get("/posts/{id}")]
// async fn get_post(web::Path(id): web::Path<i32>) -> impl Responder {
//     // database request based on specific id
//     // let post = db.get(id);
//     // HttpResponse::Ok().json(post);
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_password = env::var("POSTGRES_PASSWORD").expect("Need database password");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!("postgres://postgres:{}@localhost:5432/blogtest", db_password)).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // todo: determine if we need this, since service seems to handle loading index.html???
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
