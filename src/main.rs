mod editor;
mod home;

use clap::Parser;

use crate::editor::KynaEditor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to open in the editor
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    KynaEditor::new(args.name)?.run();

    Ok(())
}
