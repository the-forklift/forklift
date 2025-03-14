use crate::crusher::Crusher;
use crate::store::{Crate, Depencil, Kiste, Lesart};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use sicht::{SichtMap, selector::Oder};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use tar::Archive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Carriage {
    pub map: SichtMap<String, u32, Crate>,
    unresolved: Vec<(u32, u32)>,
    dependency_lookup: BTreeMap<u32, u32>,
    crate_lookup: BTreeMap<u32, String>,
}

impl Carriage {
    pub fn new(map: SichtMap<String, u32, Crate>, crate_lookup: BTreeMap<u32, String>) -> Self {
        Self {
            map,
            unresolved: Vec::default(),
            dependency_lookup: BTreeMap::default(),
            crate_lookup,
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let carriage =
            archive
                .entries()
                .unwrap()
                .fold(Option::<Carriage>::None, |mut carriage, entry| {
                    if let Ok(entry) = entry
                        && let Ok(path) = entry.path()
                        && path.extension().and_then(|x| x.to_str()).is_some()
                    {
                        match path {
                            p if p.ends_with("crates.csv") => {
                                let mut lookup = BTreeMap::default();
                                let kisten = Reader::from_reader(entry)
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

                                carriage = Some(Carriage::new(kisten, lookup));
                            }

                            p if p.ends_with("versions.csv")
                                && let Some(ref mut carr) = carriage =>
                            {
                                Reader::from_reader(entry).deserialize::<Lesart>().for_each(
                                    |dep| {
                                        if let Ok(ref d) = dep {
                                            carr.dependency_lookup
                                                .insert(d.crate_id.unwrap(), d.id);
                                        } else {
                                            todo!()
                                        }
                                    },
                                );
                            }

                            p if p.ends_with("dependencies.csv")
                                && let Some(ref mut carr) = carriage =>
                            {
                                Reader::from_reader(entry)
                                    .deserialize::<Depencil>()
                                    .for_each(|ver| {
                                        if let Ok(ref v) = ver
                                            && let Some(en) =
                                                carr.dependency_lookup.get(&v.crate_id)
                                            && let Some(cr) = carr.crate_lookup.get(&v.crate_id)
                                            && let Some(dep) = carr.crate_lookup.get(&v.id)
                                        {
                                            carr.add_dependency(
                                                *en,
                                                dep.to_string(),
                                                v.crate_id,
                                                cr.to_string(),
                                            );
                                        } else {
                                            todo!()
                                        }
                                    });
                            }

                            _ => {}
                        }
                    }

                    carriage
                });

        carriage.ok_or_else(|| todo!())
    }

    pub fn add_dependency(
        &mut self,
        krate: u32,
        krate_name: String,
        dependency: u32,
        dependency_name: String,
    ) {
        if let Some(cr) = self
            .map
            .get_with_both_keys_mut(&Oder::new(krate_name, krate))
        {
            cr.add_dependency(dependency, dependency_name);
        } else {
            self.unresolved.push((krate, dependency));
        }
    }

    pub fn search(&self, krate: &String) -> Option<&Crate> {
        self.map.get_with_base_key(krate)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Mast {}

impl Mast {
    pub fn load<P: AsRef<Path>>(load_path: P) -> Result<Carriage> {
        if let Ok(mut file) = OpenOptions::new().read(true).open("lager.fork") {
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer)?;
            Self::uncrush(buffer)
        } else {
            let contents = Carriage::unarchive(load_path)?;
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
