mod editor;
use clap::Parser;

use crate::home::home2;
mod home;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    let args = Args::parse();
    if let Some(name) = args.name {
        editor::start(name);

    }else{
        ratatui::run(home2)?;
    }
    Ok(())
}
