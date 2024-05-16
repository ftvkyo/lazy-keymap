use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};

use crate::keyboard::KeySlotId;


pub type KeymapLayerId = String;


#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    #[serde(rename = "d")]
    pub display: String,
    #[serde(rename = "c")]
    pub config: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KeymapLayer {
    /// Pretty name to be used in the reference sheet
    pub name: String,
    pub keys: HashMap<KeySlotId, KeyConfig>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Keymap {
    pub name: Option<String>,
    #[serde(rename = "for")]
    pub board: String,
    /// Additional includes for the config
    pub includes: Vec<String>,
    #[serde(with = "tuple_vec_map")]
    pub layers: Vec<(KeymapLayerId, KeymapLayer)>,
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
