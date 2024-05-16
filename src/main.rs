#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic)]
#![feature(if_let_guard, iter_map_windows)]

use anyhow::Result;

<<<<<<< HEAD
mod cli;
mod fetch;
=======
use crate::process::{Crate, Crates};
mod joystick;
>>>>>>> 0e3f969 (joystick compiles for list; initial parsing structure and parsing for list keyword)
mod process;

fn main() -> Result<()> {
   cli::init()
}
