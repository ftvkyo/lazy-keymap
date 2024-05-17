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
    /// Name of the keymap
    pub name: Option<String>,
    /// Name of the board this keymap is for
    pub board: String,
    /// Additional includes for the config
    pub includes: Option<Vec<String>>,
    /// Additional defines
    pub defines: Option<Vec<(String, String)>>,
    /// Additional nodes under the root node (verbatim)
    pub extras: Option<String>,
    /// Pairs of layer ids and layers
    #[serde(with = "tuple_vec_map")]
    pub layers: Vec<(KeymapLayerId, KeymapLayer)>,
}

impl Keymap {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading keymap from {:?}", path);

        let keymap = std::fs::read_to_string(&path).with_context(|| format!("Could not read {:?}", path))?;
        let mut keymap: Keymap = toml::from_str(&keymap).with_context(|| "Could not parse the keymap")?;

        /* ========== *
         * Validation *
         * ========== */

        // There must be no empty / whitespace config strings
        for (layer_id, layer) in &keymap.layers {
            for (key_id, key) in &layer.keys {
                if key.config.trim().is_empty() {
                    return Err(anyhow::Error::msg(format!(
                        "Layer {layer_id}, key {key_id} config is empty"
                    )));
                }
            }
        }

        /* ======== *
         * Niceness *
         * ======== */

        // Pad config strings so it's easier to read the resulting file
        for (_, layer) in &mut keymap.layers {
            for (_, key) in &mut layer.keys {
                key.config = format!("{: >8}", key.config);
            }
        }

        Ok(keymap)
    }
}
