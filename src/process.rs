use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use nohash::IntMap;
use raad::le::{R, W};
use serde_derive::Deserialize;
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashSet,
    },
    hash::BuildHasherDefault,
    io::{BufReader, Read, Write},
    ops::Deref,
};
use tar::Archive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate {
    pub name: Box<str>,
    // this is an index.
    pub dependents: HashSet<Index>,
}

pub fn get() -> Result<Crates> {
    match std::fs::File::open("db-dump.fork") {
        Ok(x) => {
            comat::cwriteln!(
                std::io::stderr(),
                "{bold_green}{:>12}{reset} `db-dump.fork`",
                "Loading"
            )?;
            load(&mut BufReader::new(x))
        }
        Err(_) if let Ok(x) = std::fs::File::open("db-dump.tar.gz") => {
            comat::cwriteln!(
                std::io::stderr(),
                "{bold_green}{:>12}{reset} `db-dump.tar.gz`",
                "Preprocessing"
            )?;
            let len = x.metadata()?.len();
            pre(&mut BufReader::new(x), len)
        }
        Err(_) => {
            comat::cwriteln!(
                std::io::stderr(),
                "{bold_green}{:>12}{reset} `db-dump.tar.gz`",
                "Downloading"
            )?;
            let (len, mut r) = download()?;
            pre(&mut r, len)
        }
    }
}

pub fn download() -> Result<(u64, impl Read)> {
    let r = ureq::get("https://static.crates.io/db-dump.tar.gz").call()?;
    Ok((
        r.header("Content-Length")
            .ok_or(anyhow!("no content length"))?
            .parse::<u64>()?,
        r.into_reader(),
    ))
}

#[allow(clippy::default_trait_access)]
pub fn load(from: &mut impl Read) -> Result<Crates> {
    if from.r::<[u8; 4]>()? != *b"FORK" {
        anyhow::bail!("wrong file type");
    }
    let cc = from.r::<u32>()?;
    if cc == 0 {
        anyhow::bail!("no crates");
    }
    let mut data = IntMap::with_capacity_and_hasher(cc as _, BuildHasherDefault::default());
    for _ in 0..cc {
        let k = from.r::<u32>()?;
        let n = from.r::<u8>()?;
        let mut out = vec![0u8; n as _];
        from.read_exact(&mut out)?;
        let name = String::from_utf8(out)?.into();
        let dc = from.r::<u16>()?;
        let mut dependents = HashSet::<Index>::with_capacity(dc as _);
        for _ in 0..dc {
            let n = from.r::<u32>()?;
            dependents.insert(Index(n));
        }

        data.insert(k, Crate { name, dependents });
    }
    Ok(data)
}

impl std::fmt::Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Crates = IntMap<u32, Crate>;

#[allow(clippy::cast_possible_truncation)]
fn write(crates: &Crates, to: &mut impl Write) -> std::io::Result<()> {
    to.w(b"FORK")?;
    to.w(crates.len() as u32)?;
    for (&k, c) in crates {
        // hash
        to.w(k)?;
        // max 64 chars
        to.w(c.name.len() as u8)?;
        to.w(c.name.as_bytes())?;
        to.w(c.dependents.len() as u16)?;
        c.dependents.iter().try_for_each(|x| to.w(x.0))?;
    }
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Index(u32);

impl Deref for Index {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index {
    pub fn dbg(self, crates: &Crates) -> impl std::fmt::Debug + '_ {
        struct D<'a>(&'a Crates, Index);
        impl<'a> std::fmt::Debug for D<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0[&self.1].dbg(self.0))
            }
        }
        D(crates, self)
    }
}

impl Crate {
    pub fn dbg<'a>(&'a self, crates: &'a Crates) -> impl std::fmt::Debug + 'a {
        struct D<'a, 'b>(&'a Crates, &'b Crate);
        impl<'a, 'b> std::fmt::Debug for D<'a, 'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} ", self.1.name)?;
                self.1
                    .dependents
                    .iter()
                    .try_for_each(|x| write!(f, "{:?}", x.dbg(self.0)))
            }
        }
        D(crates, self)
    }
}

// type Crates =
// indexmap::IndexMap<u32, Crate, std::hash::BuildHasherDefault<nohash::NoHashHasher<u32>>>;

#[allow(clippy::default_trait_access)]
pub fn pre(file: &mut impl Read, sz: u64) -> Result<Crates> {
    // currently: 144,851 crates
    let mut dat = IntMap::with_capacity_and_hasher(1_500_000, BuildHasherDefault::default());
    // currently: 1117542
    // maps the version id to the crate id because dependencies.csv is dumb
    let mut vmap = IntMap::with_capacity_and_hasher(1_500_000, BuildHasherDefault::default());
    let pb = ProgressBar::hidden();
    pb.set_length(sz);
    pb.set_prefix(comat::cformat!("{bold_green}Loading{reset}"));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:>12.cyan.bold} [{bar:20.green}] {percent}%: {wide_msg}")
            .unwrap()
            .progress_chars("-> "),
    );
    pb.set_draw_target(indicatif::ProgressDrawTarget::stderr());
    let input = pb.wrap_read(file);
    // could mmap, but speed gains are nonexistent.
    let archive = Archive::new(GzDecoder::new(input));
    for entry in { archive }.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        let Some("csv") = path.extension().and_then(|x| x.to_str()) else {
            continue;
        };
        pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());
        if path.ends_with("crates.csv") {
            crates(&mut dat, entry)?;
        } else if path.ends_with("versions.csv") {
            versions(entry, &mut vmap)?;
        } else if path.ends_with("dependencies.csv") {
            assert!(
                !vmap.is_empty(),
                "order failure (dont know if this is possible)"
            );
            dependencies(&mut dat, entry, &vmap)?;
        }
    }
    pb.finish_with_message("done");
    let out = std::fs::File::create("db-dump.fork").unwrap();
    comat::cwriteln!(
        std::io::stderr(),
        "{bold_green}{:>12}{reset} `db-dump.fork`",
        "Writing"
    )?;
    write(&dat, &mut { out })?;
    Ok(dat)
}

fn crates(crates: &mut Crates, r: impl Read) -> Result<()> {
    #[derive(Deserialize)]
    struct R {
        #[serde(rename = "id")]
        hash: u32,
        name: Box<str>,
    }
    csv::Reader::from_reader(r).deserialize().try_for_each(|x| {
        let R { hash, name } = x?;
        match crates.entry(hash) {
            Occupied(x) => ({ x }.get_mut()).name = name,
            Vacant(x) => {
                x.insert(Crate {
                    name,
                    dependents: HashSet::new(),
                });
            }
        };

        Ok(())
    })
}

fn versions(r: impl Read, versions: &mut IntMap<u32, u32>) -> Result<()> {
    #[derive(Deserialize)]
    struct Version {
        #[serde(rename = "crate_id")]
        hash: u32,
        #[serde(rename = "id")]
        version_hash: u32,
    }
    csv::Reader::from_reader(r)
        .deserialize()
        .try_for_each(|version| {
            let Version { version_hash, hash } = version?;
            assert!(versions.insert(version_hash, hash).is_none());
            Ok(())
        })
}

fn dependencies(crates: &mut Crates, r: impl Read, versions: &IntMap<u32, u32>) -> Result<()> {
    #[derive(Deserialize)]
    struct Dep {
        // #[serde(rename = "req")]
        // version: semver::VersionReq,
        #[serde(rename = "crate_id")]
        crt: u32,
        #[serde(rename = "version_id")]
        deps_on: u32,
    }
    // crate_id,default_features,explicit_name,features,id,kind,optional,req,target,version_id

    csv::Reader::from_reader(r)
        .deserialize()
        .try_for_each(|version| {
            let Dep { deps_on, crt } = version?;
            let deps_on = versions[&deps_on];
            match crates.entry(crt) {
                Occupied(x) => drop({ x }.get_mut().dependents.insert(Index(deps_on))),
                Vacant(x) => {
                    x.insert(Crate {
                        name: "".into(),
                        dependents: HashSet::from([Index(crt)]),
                    });
                }
            };
            Ok(())
        })
}
