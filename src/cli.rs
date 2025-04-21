use crate::download::{Config, Ignition};
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

    #[arg(short, long)]
    fresh: bool,
}

pub fn init() -> Result<()> {
    let args = Args::parse();
    match args {
        Args {
            package: Some(_),
            interactive: false,
            query: None,
            fresh,
        } => todo!(),
        Args {
            package: None,
            interactive: false,
            query: Some(q),
            fresh: false,
        } => {
            let accumulator: Query = QueryAccumulator::from_input(&q).try_into()?;
            let mut engine = Ignition::init(accumulator)?;
            let results = engine.run()?;
            engine.process_output()
        }
        Args {
            package: None,
            interactive: false,
            query: Some(q),
            fresh: true,
        } => {
            let accumulator: Query = QueryAccumulator::from_input(&q).try_into()?;
            let mut engine = Ignition::init_with_config(accumulator, Config::fresh())?;
            let results = engine.run()?;
            engine.process_output()
        }

        _ => todo!("no query"),
    }
}
