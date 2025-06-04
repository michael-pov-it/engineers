mod i18n;
mod views;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views::services))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
