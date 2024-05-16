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


const DIR_KEYBOARD: &'static str = "DIR_KEYBOARD";
const DIR_KEYMAP: &'static str = "DIR_KEYMAP";
const KEYMAP: &'static str = "KEYMAP";
const OUT_SVG: &'static str = "OUT_SVG";


fn try_main(args: Args) -> Result<()> {
    let mut p_keyboard: PathBuf = env::var(DIR_KEYBOARD)
        .unwrap_or_else(|_| "keyboard".into())
        .into();
    let mut p_keymap: PathBuf = env::var(DIR_KEYMAP)
        .unwrap_or_else(|_| "keymap".into())
        .into();
    let p_out_svg: PathBuf = env::var(OUT_SVG)
        .unwrap_or_else(|_| "out.svg".into())
        .into();

    let keymap_name = match (args.keymap, env::var(KEYMAP)) {
        (Some(keymap), _) => keymap,
        (None, Ok(keymap)) => keymap,
        _ => return Err(anyhow::Error::msg(format!("Neither {} env var, nor {} arg are set", KEYMAP, stringify!(args.keymap)))),
    };

    p_keymap.push(keymap_name);
    p_keymap.set_extension("toml");

    let keymap = Keymap::load(p_keymap)?;

    p_keyboard.push(&keymap.keyboard);
    p_keyboard.set_extension("toml");

    let keyboard = Keyboard::load(p_keyboard)?;

    let svg = render_svg(&keyboard, &keymap);

    info!("Saving SVG to {:?}", &p_out_svg);
    svg::save(&p_out_svg, &svg?).with_context(|| format!("Could not save SVG to {:?}", p_out_svg))?;

    Ok(())
}


fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let args = Args::parse();

    match try_main(args) {
        Ok(_) => info!("Done!"),
        Err(e) => error!("Error: {}", e),
    }
}
