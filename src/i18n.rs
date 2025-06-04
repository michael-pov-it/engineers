//! Fluent-based i18n helper (stateless version)
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "locales"]
struct Locales;

/// Build a Fluent bundle for the requested language (falls back to “sk”)
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

/// Translate a single key with no arguments
pub fn tr(bundle: &FluentBundle<FluentResource>, key: &str) -> String {
    let msg = bundle.get_message(key).expect("missing key");
    let pattern = msg.value().expect("no value");
    bundle
        .format_pattern(pattern, Some(&FluentArgs::new()), &mut vec![])
        .to_string()
}
