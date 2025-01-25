use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;
