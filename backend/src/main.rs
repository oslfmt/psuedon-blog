use actix_web::{web, App, HttpServer};
// use actix_files as fs;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use actix_cors::Cors;
use std::env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_password = env::var("POSTGRES_PASSWORD").expect("Need database password");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!("postgres://postgres:{}@localhost:5432/blogtest", db_password)).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_post)
            .service(handlers::create_post)
            .service(handlers::index) // todo: determine if we need this, since service seems to handle loading index.html???
            // TODO: put this back in once I get posts to be displayed to frontend
            // .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
