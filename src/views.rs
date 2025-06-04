//! Tera templates + Actix handlers
use crate::i18n::{get_bundle, tr};
use actix_files::Files;
use actix_web::{web, HttpResponse, Responder};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use tera::{Context, Tera};

#[derive(RustEmbed)]
#[folder = "templates"]
struct Templates;

/// One global compiled Tera instance
static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    let tpl = Templates::get("index.html.tera").expect("template missing");
    tera.add_raw_template(
        "index.html.tera",
        &String::from_utf8(tpl.data.into()).unwrap(),
    )
    .expect("invalid template");
    tera
});

/// GET /{lang}
pub async fn index(lang: web::Path<String>) -> impl Responder {
    let bundle = get_bundle(&lang);

    // keys we inject into the template (snake_case)
    let keys = [
        "page_title",
        "hero_h1",
        "hero_tagline",
        "see_offer",
    ];

    let mut ctx = Context::new();
    ctx.insert("lang", &*lang);

    for key in &keys {
        ctx.insert(*key, &tr(&bundle, key));
    }

    let html = TERA.render("index.html.tera", &ctx).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// Mount dynamic + static routes
pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{lang}").route(web::get().to(index)))
        .service(Files::new("/", "./static").prefer_utf8(true));
}
