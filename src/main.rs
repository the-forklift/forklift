#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(if_let_guard, let_chains, iter_map_windows)]

use anyhow::Result;

mod cell;
mod cli;
mod conditions;
mod crusher;
mod download;
mod fs;
mod joystick;
mod lookup;
mod store;

fn main() -> Result<()> {
    cli::init()
}
