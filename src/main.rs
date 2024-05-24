#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic)]
#![feature(if_let_guard)]

use anyhow::Result;

mod cli;
mod fetch;
mod process;

fn main() -> Result<()> {
   cli::init()
}
