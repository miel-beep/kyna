mod editor;
mod home;
mod config;

use clap::Parser;

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

    if let Some(name) = args.name {
        editor::start(name)?;
    } else {
        ratatui::run(home::home)?;
    }

    Ok(())
}
