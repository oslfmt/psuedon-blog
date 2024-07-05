use actix_web::{get, web, App, HttpResponse, Responder, HttpServer};
use actix_files as fs;

async fn index() -> Result<fs::NamedFile, std::io::Error> {
    // serve the index.html from dist directory
    fs::NamedFile::open("../frontend/dist/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // todo: determine if we need this, since service seems to handle loading index.html???
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
