use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use futures::TryStreamExt;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::models::{Post, PostFormData};

#[get("/")]
pub async fn index(pool: web::Data<Pool<Postgres>>) -> impl Responder {
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

    while let Some(post) = stream.try_next().await.expect("Failed to fetch all post metadata") {
        posts.push(post);
    }

    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post(path: web::Path<i32>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let id = path.into_inner();
    let mut content = String::new();
    let mut rows = sqlx::query("SELECT * FROM posts WHERE posts.id=$1")
        .bind(id)
        .fetch(pool.get_ref());

    while let Some(row) = rows.try_next().await.expect("Failed to fetch post.") {
        content = row.try_get("content").unwrap();
    }

    HttpResponse::Ok().json(content)
}

#[post("/thisishowidoit")]
pub async fn create_post(form: web::Form<PostFormData>, _pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let title = &form.title;
    // sqlx::query(
    //     "INSERT INTO posts_metadata (id, title, date, tag)
    //         VALUES (uuid, )"
    // ).execute(pool.get_ref()).await.unwrap();

    // sqlx::query(
    //     "INSERT INTO posts (id, content)
    //      VALUES (1, content)"
    // ).execute(pool.get_ref()).await.unwrap();

    HttpResponse::Ok().json(title)
}
