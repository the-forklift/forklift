use crate::joystick::{Query, QueryAccumulator};
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    package: Option<Box<str>>,

    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    query: Option<String>,
}

pub fn init() -> Result<()> {
    let args = Args::parse();
    match args {
        Args {
            package: Some(k),
            interactive: false,
            query: None,
        } => crate::fetch::init(&k),
        Args {
            package: None,
            interactive: false,
            query: Some(q),
        } => {
            let accumulator: Query = QueryAccumulator::from_input(&q).try_into()?;
            accumulator.parse()
        }
        _ => todo!("no query"),
    }
}
