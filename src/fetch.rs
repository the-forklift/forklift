use anyhow::{anyhow, Result};
use std::{collections::HashSet, env};

use crate::process::{Crate, Crates};

pub fn init(crate_name: &str) -> Result<()> {
    let crates = crate::process::get()?;
    let mut out: HashSet<Box<str>> = HashSet::new();
    #[recursive::recursive]
    fn see(out: &mut HashSet<Box<str>>, what: &Crate, from: &Crates, dot: bool) {
        let first = out.insert(what.name.clone());
        for dep in &what.dependents {
            if first && dot {
                println!("\t\"{}\" -- \"{}\";", what.name, &from[dep].name);
            }
            see(out, &from[dep], from, dot);
        }
    }
    let first = crates
        .values()
        .find(|x| *x.name == *crate_name)
        .ok_or(anyhow!("couldnt find crate"))?;
    if let Some("dot") = env::args().nth(2).as_deref() {
        println!("graph x {{");
        println!("\t{} [shape=box];", first.name);
        see(&mut out, first, &crates, true);
        println!("}}");
    } else {
        see(&mut out, first, &crates, false);
    }
    eprintln!("found {} reverse dependencies!", out.len());
    Ok(())
}
