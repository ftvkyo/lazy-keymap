use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to the keymap description file
    pub keymap: PathBuf,
}
