use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    krate: Option<Box<str>>,

    #[arg(short, long)]
    interactive: bool,
}

pub fn init() -> Result<()> {
    let args = Args::parse();
    match args {
        Args {
            krate: Some(k),
            interactive: false,
        } => crate::fetch::init(&k),
        _ => todo!(),
    }
}
