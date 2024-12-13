#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(if_let_guard, let_chains, iter_map_windows)]

use anyhow::Result;

mod cli;
mod conditions;
mod download;
mod fetch;
mod fs;
mod joystick;
mod process;
mod store;

fn main() -> Result<()> {
    cli::init()
}
