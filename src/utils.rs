use std::sync::OnceLock;
use rand::distr::Alphanumeric;
use rand::Rng;
use regex::Regex;

pub fn slugify(text: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"[^\w\s-]").unwrap());
    let cleaned = re.replace_all(text, "");
    cleaned.trim()
        .to_lowercase()
        .replace(' ', "-")
        .replace("--", "-")
}

pub fn generate_unique_slug(title: &str) -> String
{
    let base_slug = slugify(title);
    let suffix = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();
    format!("{}-{}", base_slug, suffix.to_lowercase())
}

pub fn generate_password(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}