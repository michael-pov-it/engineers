//! Fluent-based i18n helper (stateless version)
use fluent_bundle::{FluentBundle, FluentResource};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "locales"]
struct Locales;

pub fn get_bundle(lang: &str) -> FluentBundle<FluentResource> {
    let lang_id: LanguageIdentifier = lang
        .parse()
        .unwrap_or_else(|_| "sk".parse().expect("valid default langid"));

    let mut bundle = FluentBundle::new(vec![lang_id]);

    let ftl = Locales::get(&format!("{lang}.ftl")).expect("missing locale file");
    let res = FluentResource::try_new(String::from_utf8(ftl.data.into()).unwrap())
        .expect("invalid FTL");

    bundle.add_resource(res).unwrap();
    bundle
}

pub fn tr(bundle: &FluentBundle<FluentResource>, key: &str) -> String {
  match bundle.get_message(key).and_then(|m| m.value()) {
      Some(pattern) => bundle.format_pattern(pattern, None, &mut vec![]).to_string(),
      None => format!("⟪{}⟫", key),   // fallback placeholder
  }
}
