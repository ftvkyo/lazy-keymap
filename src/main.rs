use clap::Parser;
use keymap::Keymap;

pub mod args;
pub mod keymap;

fn main() {
    let args = args::Args::parse();

    let keymap = std::fs::read_to_string(args.keymap).expect("Could not read the file");
    let keymap: Keymap = toml::from_str(&keymap).expect("Could not parse the keymap");

    svg::save("out.svg", &keymap.svg()).expect("Could not save SVG");
}
