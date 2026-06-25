#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(associated_type_defaults)]

use anyhow::Result;

mod carriage;
mod cell;
mod cli;
mod conditions;
mod crusher;
mod download;
mod fs;
mod joystick;
mod lookup;
mod serproxy;
mod store;

fn main() -> Result<()> {
    cli::init()
}
