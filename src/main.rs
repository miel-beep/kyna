mod editor;
mod buffer;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::new())]
    name: String,
}

fn main() {
    let args = Args::parse();

    editor::start(args.name);
}
