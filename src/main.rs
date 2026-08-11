#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]

use crate::error::Error;

mod carriage;
mod cell;
mod cli;
mod conditions;
mod crusher;
mod download;
mod error;
mod fs;
mod joystick;
mod serproxy;
mod store;

fn main() -> Result<(), Error> {
    cli::init()
}
