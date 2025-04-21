use crate::cell::SichtCell;
use crate::crusher::Crusher;
use crate::download::Config;
use crate::lookup::Lookup;
use crate::store::{Crate, Depencil, Kiste, Lesart};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use sicht::{SichtMap, selector::Oder};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::{Archive, Entry};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Carriage {
    pub map: SichtCell<String, u32, Crate>,
    unresolved: Vec<(u32, u32)>,
}

impl Carriage {
    pub fn new(map: SichtCell<String, u32, Crate>) -> Self {
        Self {
            map,
            unresolved: Vec::default(),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let (carriage, _) = archive.entries().unwrap().fold(
            (Option::<Carriage>::None, Lookup::default()),
            |(mut carriage, mut lookup), entry| {
                if let Ok(entry) = entry
                    && let Ok(path) = entry.path()
                    && path.extension().and_then(|x| x.to_str()).is_some()
                {
                    match path {
                        p if p.ends_with("crates.csv") => {
                            let (carr, lu) = Self::process_crates(entry);
                            carriage = Some(carr);
                            lookup = lu;
                        }
                        p if let Some(ref carriage) = carriage
                            && p.ends_with("versions.csv") =>
                        {
                            carriage.process_versions(entry, &mut lookup);
                        }
                        p if let Some(ref carriage) = carriage
                            && p.ends_with("dependencies.csv") =>
                        {
                            carriage.process_dependencies(entry, &mut lookup);
                        }
                        _ => {}
                    }
                } else {
                }
                (carriage, lookup)
            },
        );

        Ok(carriage.unwrap())
    }

    pub fn process_crates(entry: impl Read) -> (Self, Lookup) {
        let mut lookup = BTreeMap::default();
        let map = Reader::from_reader(entry)
            .deserialize::<Kiste>()
            .map(|cr| {
                if let Ok(c) = cr {
                    lookup.insert(c.id, c.name.clone());
                    (Oder::new(c.name.clone(), c.id), Crate::new(c))
                } else {
                    todo!()
                }
            })
            .collect();

        (
            Carriage::new(SichtCell::new(map)),
            Lookup::with_krate(lookup),
        )
    }

    #[allow(clippy::unused_self)]
    pub fn process_versions(&self, entry: impl Read, lookup: &mut Lookup) {
        Reader::from_reader(entry)
            .deserialize::<Lesart>()
            .for_each(|ver| {
                if let Ok(ref v) = ver
                    && let Some(crate_id) = v.crate_id
                {
                    lookup.insert_dependency_relation(v.id, crate_id);
                } else {
                    todo!()
                }
            });
    }

    pub fn process_dependencies(&self, entry: impl Read, lookup: &mut Lookup) {
        Reader::from_reader(entry)
            .deserialize::<Depencil>()
            .for_each(|dep| {
                if let Ok(ref d) = dep {
                    let krate_name = lookup.get_crate_name(d.crate_id);
                    let dependency = lookup
                        .get_dependency_relation_for_version(d.version_id)
                        .copied();
                    if let Some(krate_name) = krate_name
                        && let Some(dependency) = dependency
                        && let Some(dependency_name) = lookup.get_crate_name(dependency)
                    {
                        self.add_dependency(
                            d.crate_id,
                            krate_name.to_owned(),
                            dependency,
                            dependency_name.to_owned(),
                        );
                    }
                }
            });
    }

    pub fn add_dependency(
        &self,
        krate: u32,
        krate_name: String,
        dependency: u32,
        dependency_name: String,
    ) {
        if let Some(cr) = self
            .map
            .borrow_mut()
            .get_with_both_keys(&Oder::new(krate_name, krate))
        {
            cr.add_dependency(dependency, dependency_name);
        } else {
            todo!()
        }
    }

    pub fn search(&self, krate: &String) -> Option<Crate> {
        self.map.borrow().get_with_base_key(krate).cloned()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Mast {
    path: PathBuf,
    #[serde(skip)]
    config: Config,
}

impl Mast {
    pub fn path<P: AsRef<Path>>(load_path: P) -> Self {
        Mast {
            path: load_path.as_ref().to_owned(),
            config: Config::default(),
        }
    }

    pub fn config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn load(&mut self) -> Result<Carriage> {
        if !self.config.fresh
            && let Ok(mut file) = OpenOptions::new().read(true).open("lager.fork")
        {
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer)?;
            Self::uncrush(buffer)
        } else {
            let contents = Carriage::unarchive(&self.path)?;
            let _ = Self::store_contents(&contents);
            Ok(contents)
        }
    }

    pub fn store_contents(contents: &Carriage) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("lager.fork")?;
        ron::ser::to_writer(file, &contents)?;
        Ok(())
    }
}
