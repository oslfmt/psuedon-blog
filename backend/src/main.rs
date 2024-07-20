use actix_web::{get, web, App, HttpResponse, Responder, HttpServer};
use actix_files as fs;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres, Row};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use futures::TryStreamExt;
use actix_cors::Cors;

use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: i32,
    title: String,
    date: chrono::NaiveDate,
    tag: String,
}

#[get("/")]
async fn index(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    // fetch all posts metadata
    let mut stream = sqlx::query("SELECT * from posts_metadata")
        .map(|row: PgRow| {
            Post {
                id: row.get("id"),
                title: row.get("title"),
                date: row.get("date"),
                tag: row.get("tag"),
            }
        })
        .fetch(pool.get_ref());

    let mut posts = Vec::new();

    while let Some(post) = stream.try_next().await.expect("Failed to fetch post") {
        posts.push(post);
    }

    HttpResponse::Ok().json(posts)
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

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .app_data(web::Data::new(pool.clone()))
            .service(index) // todo: determine if we need this, since service seems to handle loading index.html???
            // TODO: put this back in once I get posts to be displayed to frontend
            // .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
