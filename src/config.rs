use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(flatten)]
    pub lists: HashMap<String, PokemonList>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonList {
    pub pokemon: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        if let Some(path) = get_config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Config::default()
    }

    pub fn expand_list(&self, list_name: &str) -> Option<Vec<String>> {
        self.lists.get(list_name).map(|list| list.pokemon.clone())
    }
}

fn get_config_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(".config").join("pokeget").join("config.toml"))
}
