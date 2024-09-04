use actix_files::NamedFile;
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use futures::TryStreamExt;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use actix_session::Session;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::models::{Post, PostFormData, LoginData};

pub async fn static_file_fallback(_req: HttpRequest) -> Result<NamedFile, Error> {
    Ok(NamedFile::open("../frontend/dist/index.html")?)
}

#[get("/api/posts")]
pub async fn get_posts(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    // fetch all posts metadata
    let mut stream = sqlx::query("SELECT * from posts_metadata ORDER BY date DESC")
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

    while let Some(post) = stream.try_next().await.expect("Failed to fetch all post metadata") {
        posts.push(post);
    }

    HttpResponse::Ok().json(posts)
}

#[get("/api/posts/{id}")]
pub async fn get_post(path: web::Path<String>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let id = path.into_inner();
    let mut content = String::new();
    let mut title = String::new();
    let mut date = chrono::NaiveDate::MAX;

    let mut rows = sqlx::query(
        "SELECT title, date, content FROM posts
        INNER JOIN posts_metadata ON posts.id = posts_metadata.id
        WHERE posts.id=$1"
    )
    .bind(id)
    .fetch(pool.get_ref());

    while let Some(row) = rows.try_next().await.expect("Failed to fetch post.") {
        content = row.try_get("content").unwrap();
        title = row.get("title");
        date = row.get("date");
    }

    HttpResponse::Ok().json((content, title, date))
}

#[post("/api/login")]
pub async fn verify_login(login_data: web::Json<LoginData>, session: Session) -> impl Responder {
    let username = &login_data.username;
    // TODO: move this to be read from a file or something
    let hash = "$argon2id$v=19$m=19456,t=2,p=1$cQ36/FlQ8l56gN/DFXUfxw$zSWMz1ii2rSyoAE9sYIEIbcjj32g0bM/fsfFOnZC/9U";
    let parsed_hash = PasswordHash::new(hash).unwrap();

    if Argon2::default().verify_password(login_data.password.as_bytes(), &parsed_hash).is_ok() {
        // start a session for admin
        session.insert("user_name", username).unwrap();
        HttpResponse::Ok().json("Login success!")
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

#[get("/api/authenticate")]
pub async fn authenticate(session: Session) -> impl Responder {
    if let Some(_user_name) = session.get::<String>("user_name").unwrap() {
        HttpResponse::Ok()
    } else {
        HttpResponse::Unauthorized()
    }
}

#[post("/api/thisishowidoit")]
pub async fn create_post(form: web::Form<PostFormData>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let uuid = Uuid::new_v4().to_string();

    let title = &form.title;
    let content = &form.content;
    let date = &form.date;
    let tag = &form.tag;
    
    sqlx::query(
        "INSERT INTO posts_metadata (id, title, date, tag)
            VALUES ($1, $2, $3, $4)"
    )
    .bind(&uuid)
    .bind(title)
    .bind(date)
    .bind(tag)
    .execute(pool.get_ref()).await.unwrap();

    sqlx::query(
        "INSERT INTO posts (id, content)
         VALUES ($1, $2)"
    )
    .bind(&uuid)
    .bind(content)
    .execute(pool.get_ref()).await.unwrap();

    HttpResponse::Ok().json("Post added to database!")
}
