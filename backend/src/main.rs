use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{cookie::{Key, SameSite}, web, App, HttpServer};
// use actix_files as fs;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use actix_cors::Cors;
use std::env;

mod handlers;
mod models;

fn session_middleware_local_test() -> SessionMiddleware<CookieSessionStore> {
    let key = Key::generate();

    SessionMiddleware::builder(
        CookieSessionStore::default(),
        key.clone(),
    )
    .cookie_secure(false)
    .cookie_same_site(SameSite::Lax)
    .build()
}

fn session_middleware_production() -> SessionMiddleware<CookieSessionStore> {
    // TODO: get from env file
    let key = Key::generate();

    SessionMiddleware::new(
        CookieSessionStore::default(),
        key.clone(),
    )
}

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
            .wrap(session_middleware_local_test())
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_post)
            .service(handlers::create_post)
            .service(handlers::verify_login)
            .service(handlers::authenticate)
            .service(handlers::index) // todo: determine if we need this, since service seems to handle loading index.html???
            // TODO: put this back in once I get posts to be displayed to frontend
            // .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
