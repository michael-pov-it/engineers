mod i18n;
mod views;

use actix_web::{App, HttpServer};
use actix_web::middleware::DefaultHeaders;           // keep only if you add the header

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // uncomment the next line if you really want the noindex header
            .wrap(DefaultHeaders::new().add(("X-Robots-Tag", "noindex, nofollow")))
            .configure(views::services)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
