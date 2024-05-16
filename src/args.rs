#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to the keymap description file
    pub keymap: Option<String>,
    /// Enable ids on key slots in the SVG refcard
    #[arg(short, long)]
    pub ids: bool,
}
