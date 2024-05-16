use std::path::Path;

use anyhow::{Context, Result};
use log::info;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Keymap {
    pub name: Option<String>,
    #[serde(rename = "for")]
    pub keyboard: String,
}

impl Keymap {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading keymap from {:?}", path);

        let keymap = std::fs::read_to_string(&path).with_context(|| format!("Could not read {:?}", path))?;
        let keymap = toml::from_str(&keymap).with_context(|| "Could not parse the keymap")?;

        Ok(keymap)
    }
}
