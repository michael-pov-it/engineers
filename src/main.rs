use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // English first  (exact prefix match)
            .service(Files::new("/en", "./static/en").index_file("index.html"))
            // Slovak explicit
            .service(Files::new("/sk", "./static/sk").index_file("index.html"))
            // Root → Slovak  (catch-all must be LAST)
            .service(Files::new("/",   "./static/sk").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
