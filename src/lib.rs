use rust_embed::RustEmbed;

pub mod cli;
pub mod config;
pub mod list;
pub mod pokemon;
mod random;
pub mod sprites;

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]
pub struct Data;
