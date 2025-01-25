use rust_embed::RustEmbed;

#[derive(Clone, RustEmbed)]
#[folder = "public"]
pub struct Public;
