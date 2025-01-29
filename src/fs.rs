use crate::store::{Crate, Depencil, Kiste, Lesart};
use csv::Reader;
use flate2::read::GzDecoder;
use sicht::{selector::Oder, SichtMap};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::ptr::NonNull;
use tar::Archive;

#[derive(Debug)]
pub struct Carriage {
    pub map: SichtMap<String, u32, Crate>,
    unresolved: Vec<u32>,
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
                                    .flat_map(|cr| {
                                        cr.map(|c| {
                                            lookup.insert(c.id, c.name.clone());
                                            (Oder::new(c.name.clone(), c.id), Crate::new(c))
                                        })
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
                                            carr.dependency_lookup.insert(d.id, d.crate_id);
                                        }
                                    },
                                );
                            }

                            p if p.ends_with("dependencies.csv")
                                && let Some(ref mut carr) = carriage =>
                            {
                                Reader::from_reader(entry)
                                    .deserialize::<Depencil>()
                                    .enumerate()
                                    .for_each(|(k, ver)| {
                                        if let Ok(ref v) = ver
                                            && let Some(en) = carr.dependency_lookup.get(&v.id)
                                            && let Some(cr) = carr.crate_lookup.get(&v.crate_id)
                                            && let Some(dep_name) = carr.crate_lookup.get(en)
                                        {
                                            carr.add_dependency(
                                                *en,
                                                v.crate_id,
                                                cr.to_string(),
                                                dep_name.to_string(),
                                            );
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

    pub fn add_dependency(&mut self, krate: u32, dependency: u32, name: String, dep_name: String) {
        let dep = {
            let Some(r) = self.resolve_dependency(dep_name, dependency) else {
                return;
            };
            NonNull::new(std::ptr::from_ref(r).cast_mut()).unwrap()
        };

        if let Some(cr) = self.map.get_with_both_keys_mut(&Oder::new(name, krate)) {
            cr.add_dependency(krate, dep);
        } else {
            todo!()
        }
    }

    pub fn resolve_dependency(&mut self, name: String, dependency: u32) -> Option<&Crate> {
        if let Some(k) = self
            .map
            .get_with_both_keys_mut(&Oder::new(name, dependency))
        {
            Some(k)
        } else {
            self.unresolved.push(dependency);
            None
        }
    }

    pub fn add_version(&mut self, version: &Lesart) {}
}
