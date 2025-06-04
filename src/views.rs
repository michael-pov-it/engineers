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
      "cta_h2",
      "contact_us",
      "copyright_text",
      "hello_email",
      "punuka_card1_h2",
      "punuka_card1_p",
      "punuka_card2_h2",
      "punuka_card2_p",
      "punuka_card3_h2",
      "punuka_card3_p",
      "key_aspects_h2",
      "key_aspects_ol_li1_strong",
      "key_aspects_ol_li1",
      "key_aspects_ol_li2_strong",
      "key_aspects_ol_li2",
      "key_aspects_ol_li3_strong",
      "key_aspects_ol_li3",
      "key_aspects_ol_li4_strong",
      "key_aspects_ol_li4",
      "key_aspects_ol_li5_strong",
      "key_aspects_ol_li5",
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
    cfg
      .service(Files::new("/assets", "./static").prefer_utf8(true))  // /assets/…
      .service(web::resource("/{lang}").route(web::get().to(index))) // /en, /sk …
      .route("/", web::get().to(redirect_root));                     // optional /
}

async fn redirect_root() -> impl actix_web::Responder {
  actix_web::HttpResponse::Found()
      .append_header(("Location", "/sk"))   // default language
      .finish()
}
