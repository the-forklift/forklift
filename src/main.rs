#![feature(let_chains)]
use anyhow::{anyhow, Result};
use fimg::{Image, ReadPng};
use std::{collections::HashSet, env, io::Write, process::Stdio};

use crate::process::{Crate, Crates};
mod process;

fn main() -> Result<()> {
    let crate_name = env::args().nth(1).expect("Usage: forklift [crate] (dot)");
    let crates = process::get()?;
    let out: HashSet<Box<str>>;
    #[recursive::recursive]
    fn see(
        out: &mut HashSet<Box<str>>,
        what: &Crate,
        from: &Crates,
        f: &mut impl Write,
    ) -> std::io::Result<()> {
        let first = out.insert(what.name.clone());
        for dep in &what.dependents {
            if first {
                writeln!(f, "\t\"{}\" -- \"{}\";", what.name, &from[&dep].name)?;
            }
            see(out, &from[&dep], from, f)?;
        }
        Ok(())
    }
    fn walk(
        first: &Crate,
        crates: &Crates,
        f: &mut impl Write,
    ) -> std::io::Result<HashSet<Box<str>>> {
        let mut out = HashSet::new();
        writeln!(f, "graph x {{")?;
        writeln!(f, "\t{} [shape=box];", first.name)?;
        see(&mut out, first, &crates, f)?;
        writeln!(f, "}}")?;
        Ok(out)
    }
    let first = crates
        .values()
        .find(|x| &*x.name == &*crate_name)
        .ok_or(anyhow!("couldnt find crate"))?;
    if let Some("dot") = env::args().nth(2).as_deref() {
        #[cfg(unix)]
        let tty = unsafe { libc::isatty(libc::STDOUT_FILENO) } == 1;
        #[cfg(not(unix))]
        let tty = true;
        if tty {
            let mut proc = std::process::Command::new("dot")
                .args(["-Tpng"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            let mut stdin = proc.stdin.take().unwrap();
            let mut stdout = proc.stdout.take().unwrap();
            out = walk(first, &crates, &mut stdin)?;
            let i = Image::<_, 3>::read(&mut stdout)?;
            println!("{}", i.as_ref());
        } else {
            out = walk(first, &crates, &mut std::io::stdout())?;
        }
    } else {
        out = walk(first, &crates, &mut std::io::sink()).unwrap();
    }
    eprintln!("found {} reverse dependencies!", out.len());
    Ok(())
}
