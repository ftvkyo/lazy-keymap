use std::{collections::HashMap, path::Path};

use anyhow::{Result, Context};
use log::info;
use serde::Deserialize;


pub type KeySlotId = String;

#[derive(Debug, Deserialize)]
pub struct KeySlot {
    /// Position of the top-left corner of the key (before rotation)
    #[serde(rename = "pos")]
    pub position: (f32, f32),
    /// Size of the key
    pub size: Option<(f32, f32)>,
    /// Rotation of the key (clockwise, degrees)
    pub angle: Option<f32>,
}

pub type KeyboardFileTemplate = String;
pub type KeyboardLayerTemplate = String;

#[derive(Debug, Deserialize)]
pub struct KeymapTemplates {
    pub file: KeyboardFileTemplate,
    pub layer: KeyboardLayerTemplate,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub name: String,
    pub slots: HashMap<KeySlotId, KeySlot>,
    pub templates: KeymapTemplates,
}

impl Keyboard {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading keyboard from {:?}", path);

        let keyboard = std::fs::read_to_string(&path).with_context(|| format!("Could not read {:?}", path))?;
        let keyboard = toml::from_str(&keyboard).with_context(|| "Could not parse the keyboard")?;

        Ok(keyboard)
    }
}
