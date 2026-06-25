use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Cdv, Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use sicht::SichtMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use tar::Archive;

#[derive(Clone, Debug, Default)]
pub struct Carriage {
    pub map: SichtCell<SichtMap<u32, String, Crate>>,
    pub traversed: SichtCell<SichtMap<u32, String, Skid>>,
    pub lookup: SichtCell<Lookup>,
}

impl<'a> Carriage {
    pub fn new(map: SichtCell<SichtMap<u32, String, Crate>>, lookup: Lookup) -> Self {
        Self {
            map,
            traversed: SichtCell::default(),
            lookup: SichtCell::new(lookup),
        }
    }

    pub fn from_map(map: SichtMap<u32, String, Crate>) -> Self {
        Self {
            map: SichtCell::new(map),
            traversed: SichtCell::default(),
            lookup: SichtCell::default(),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let cdv = archive.entries()?.fold(Cdv::default(), |mut cdv, e| {
            match e {
                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("crates.csv") =>
                {
                    cdv.crates = Reader::from_reader(e)
                        .deserialize::<Kiste>()
                        .filter_map(Result::ok)
                        .map(|cr| {
                            let name = cr.name.clone();
                            (cr.id, name, Crate::new(cr.clone()))
                        })
                        .collect::<SichtMap<u32, String, Crate>>();
                }
                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("dependencies.csv") =>
                {
                    cdv.dependencies = Reader::from_reader(e)
                        .deserialize::<Depencil>()
                        .filter_map(Result::ok)
                        .fold(BTreeMap::<u32, Vec<u32>>::default(), |mut deps, dep| {
                            deps.entry(dep.version_id)
                                .and_modify(|v| v.push(dep.crate_id))
                                .or_insert_with(|| vec![dep.crate_id]);
                            deps
                        });
                }

                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("versions.csv") =>
                {
                    cdv.versions = Reader::from_reader(e)
                        .deserialize::<Lesart>()
                        .filter_map(Result::ok)
                        .filter_map(|ver| ver.crate_id.map(|c_id| (ver.id, c_id)))
                        .collect::<BTreeMap<u32, u32>>();
                }

                _ => {}
            }

            cdv
        });
        Ok(cdv.process_to_carriage())
    }

    #[allow(clippy::needless_for_each)]
    pub fn process_dependencies(
        &self,
        dependencies: &BTreeMap<u32, Vec<u32>>,
        crates_list: &BTreeMap<u32, u32>,
    ) {
        dependencies.iter().for_each(|(version_id, crates)| {
            crates
                .iter()
                .for_each(|ver| self.add_dependency_to_crate(*version_id, *ver, crates_list));
        });
    }

    pub fn add_dependency_to_crate(
        &self,
        version_id: u32,
        crate_id: u32,
        crates: &BTreeMap<u32, u32>,
    ) {
        let d_id = crates.get(&version_id).unwrap();
        let map = self.map.borrow();
        let krate = map.get_with_base_key(&crate_id).unwrap();
        let dependency = map.get_with_base_key(d_id).unwrap();
        krate.add_dependency(dependency);
    }

    pub fn search(&self, krate: &String) -> Option<UnrolledCrate> {
        let krate_id = self.lookup.borrow();
        let krate_id = krate_id.get_crate_id(krate)?;
        let root = self.map.borrow().get(krate_id).cloned();
        dbg!(&root);
        root.map(|r| self.generate_from_crate(r))
    }

    pub fn generate_from_crate(&self, krate: Crate) -> UnrolledCrate {
        UnrolledCrate {
            crate_id: krate.krate.id,
            name: krate.krate.name,
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(krate, _)| self.generate_if_not_traversed(*krate))
                .collect(),
        }
    }

    pub fn generate_from_crate_name(&'a self, krate_name: &String) -> Option<UnrolledCrate> {
        let map = self.map.borrow();
        let krate = map.get_with_outer_key(krate_name)?;
        Some(UnrolledCrate {
            crate_id: krate.krate.id,
            name: krate_name.to_owned(),
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(krate, _)| self.generate_if_not_traversed(*krate))
                .collect(),
        })
    }

    pub fn generate_if_not_traversed(&self, crate_id: u32) -> Option<UnrolledCrate> {
        let lookup = self.lookup.borrow();
        let krate = lookup.get_crate_name(crate_id)?;
        if self.traversed.borrow().contains_key(&crate_id) {
            Some(UnrolledCrate {
                crate_id: crate_id.to_owned(),
                name: krate.to_owned(),
                dependents: Vec::default(),
            })
        } else {
            self.generate_from_crate_name(krate)
        }
    }
}
