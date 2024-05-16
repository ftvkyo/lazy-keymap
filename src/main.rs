use std::{env, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};

pub mod args;
pub mod keyboard;
pub mod keymap;
pub mod render;

use args::Args;
use keyboard::Keyboard;
use keymap::Keymap;
use render::render_svg;

use crate::render::render_config;


const DIR_KEYBOARD: &'static str = "DIR_KEYBOARD";
const DIR_KEYMAP: &'static str = "DIR_KEYMAP";

const OUT_SVG: &'static str = "OUT_SVG";
const OUT_CONFIG: &'static str = "OUT_CONFIG";

const KEYMAP: &'static str = "KEYMAP";


fn try_main(args: Args) -> Result<()> {
    let mut p_keyboard: PathBuf = env::var(DIR_KEYBOARD)
        .unwrap_or_else(|_| "keyboard".into())
        .into();
    let mut p_keymap: PathBuf = env::var(DIR_KEYMAP)
        .unwrap_or_else(|_| "keymap".into())
        .into();
    let p_out_svg: PathBuf = env::var(OUT_SVG)
        .unwrap_or_else(|_| "out/reference.svg".into())
        .into();
    let p_out_config: PathBuf = env::var(OUT_CONFIG)
        .unwrap_or_else(|_| "out/config.keymap".into())
        .into();

    let env_keymap = env::var(KEYMAP);

    let keymap_name = match (&args.keymap, &env_keymap) {
        (Some(keymap), _) => keymap,
        (None, Ok(keymap)) => keymap,
        _ => return Err(anyhow::Error::msg(format!("Neither {} env var, nor {} arg are set", KEYMAP, stringify!(args.keymap)))),
    };

    p_keymap.push(keymap_name);
    p_keymap.set_extension("toml");

    let keymap = Keymap::load(p_keymap)?;

    p_keyboard.push(&keymap.board);
    p_keyboard.set_extension("toml");

    let keyboard = Keyboard::load(p_keyboard)?;

    let svg = render_svg(&keyboard, &keymap, &args)?;
    let config = render_config(&keyboard, &keymap)?;

    if let Some(parent) = p_out_svg.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if let Some(parent) = p_out_config.parent() {
        std::fs::create_dir_all(parent)?;
    }

    info!("Saving SVG to {:?}", &p_out_svg);
    svg::save(&p_out_svg, &svg).with_context(|| format!("Could not save SVG to {:?}", p_out_svg))?;

    info!("Saving config to {:?}", &p_out_config);
    std::fs::write(&p_out_config, &config).with_context(|| format!("Could not save config to {:?}", p_out_config))?;

    Ok(())
}


fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let args = Args::parse();

    match try_main(args) {
        Ok(_) => info!("Done!"),
        Err(e) => error!("\n{:#}", e),
    }
}
