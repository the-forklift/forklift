use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Cdv, Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::{Result, anyhow};
use csv::Reader;
use flate2::read::GzDecoder;
use serde::Deserialize;
use sicht::SichtMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use tar::Archive;

#[derive(Clone, Debug, Default, Deserialize)]
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
                            deps.entry(dep.id)
                                .and_modify(|v| v.push(dep.version_id))
                                .or_insert(vec![dep.version_id]);
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
        cdv.process_to_carriage()
    }

    pub fn process_dependencies(
        &self,
        dependencies: &BTreeMap<u32, Vec<u32>>,
        version_list: &BTreeMap<u32, u32>,
    ) -> Result<(), anyhow::Error> {
        dependencies
            .iter()
            .try_for_each(|(crate_id, versions)| {
                versions
                    .iter()
                    .try_for_each(|ver| self.add_dependency_to_crate(*crate_id, *ver, version_list))
            })
            .ok_or_else(|| anyhow!("todo"))
    }

    pub fn add_dependency_to_crate(
        &self,
        crate_id: u32,
        version_id: u32,
        versions: &BTreeMap<u32, u32>,
    ) -> Option<()> {
        let d_id = versions.get(&version_id)?;
        let map = self.map.borrow();
        let krate = map.get_with_base_key(&crate_id)?;
        let dependency = map.get_with_base_key(d_id)?;
        krate.add_dependency(dependency);
        Some(())
    }

    pub fn add_dependency(&'a self, krate: u32, dependency: u32, dependency_name: &str) {
        if let Some(cr) = self.map.borrow_mut().get(&krate) {
            cr.add_dependency_with_fields(dependency, dependency_name);
        } else {
            todo!()
        }
    }

    pub fn search(&self, krate: &String) -> Option<UnrolledCrate> {
        let krate_id = self.lookup.borrow();
        let krate_id = krate_id.get_crate_id(krate)?;
        let root = self.map.borrow().get(krate_id).cloned();
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
