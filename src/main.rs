#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(if_let_guard, iter_map_windows)]

use anyhow::Result;

mod cli;
mod fetch;
mod joystick;
mod process;

fn main() -> Result<()> {
    cli::init()
}
