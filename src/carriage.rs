use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use kuh::{Kuh, Derow};
use serde::{Deserialize, Serialize};
use sicht::selector::Oder;
use std::collections::{BTreeMap, btree_map::IntoIter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Carriage<'a> {
    pub map: SichtCell<'a, String, u32, Crate>,
    pub traversed: SichtCell<'a, String, u32, Skid>,
    pub lookup: Lookup,
}

impl<'a> Carriage<'a> {
    pub fn new(map: SichtCell<'a, String, u32, Crate>) -> Self {
        Self {
            map,
            traversed: SichtCell::default(),
            lookup: Lookup::default(),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let order = archive.entries().unwrap().filter_map(|e| {
            match e {
                Ok(e) if let Ok(p) = e.path() && p.ends_with("crates.csv") => Some((0, e)),
                Ok(e) if let Ok(p) = e.path() && p.ends_with("versions.csv") => Some((1, e)),
                Ok(e) if let Ok(p) = e.path() && p.ends_with("dependencies.csv") => Some((2, e)),
                _ => None
            }
        }).collect::<BTreeMap<usize, _>>();

        let mut order = order.into_iter();
        let (_, ent) = order.next().unwrap();


        let (Some(carriage), lookup) = Self::process_crates(ent) else {
            unreachable!()
        };

        carriage.process_crate_information(order);

        Ok(carriage)

    }

    fn process_crate_information(&'a self, order: IntoIter<usize, impl Read>) {

        order.for_each(|(i, ent)| {
           match i {
               1 => {
                   self.process_dependencies(ent);
                }
               2 => {
                   self.process_versions(ent)
               }
               _ => unreachable!(),

           };

        });

    }
    

    pub fn process_crates(entry: impl Read) -> (Option<Self>, Lookup) {
        let mut lookup = BTreeMap::default();
        let map = Reader::from_reader(entry)
            .deserialize::<Kiste>()
            .map(|cr| {
                if let Ok(c) = cr {
                    lookup.insert(c.id, c.name.clone());
                    (
                        Oder::new_with_kuh(Kuh::Borrowed(&c.name), Kuh::Owned(c.id)),
                        Crate::new(c.to_owned()),
                    )
                } else {
                    todo!()
                }
            })
            .collect();

        (
            Some(Carriage::new(SichtCell::new(map))),
            Lookup::with_krate(lookup),
        )
    }

    #[allow(clippy::unused_self)]
    pub fn process_versions(&self, entry: impl Read) {
        Reader::from_reader(entry)
            .deserialize::<Lesart>()
            .for_each(|ver| {
                if let Ok(ref v) = ver
                    && let Some(crate_id) = v.crate_id
                {
                    self.lookup.insert_dependency_relation(v.id, crate_id);
                } else {
                    todo!()
                }
            });
    }

    pub fn process_dependencies(&'a self, entry: impl Read) {
        Reader::from_reader(entry)
            .deserialize::<Depencil>()
            .for_each(|dep| {
                if let Ok(ref d) = dep {
                    let krate_name = self.lookup.get_crate_name(d.crate_id);
                    let dependency = self.lookup
                        .get_dependency_relation_for_version(d.version_id)
                        .copied();
                    if let Some(krate_name) = krate_name
                        && let Some(dependency) = dependency
                        && let Some(dependency_name) = self.lookup.get_crate_name(dependency)
                    {
                        self.add_dependency(
                            d.crate_id,
                            krate_name,
                            dependency,
                            dependency_name.to_owned(),
                        );
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            });
    }

    pub fn add_dependency(
        &self,
        krate: u32,
        krate_name: &'a str,
        dependency: u32,
        dependency_name: String,
    ) {
        if let Some(cr) = self
            .map
            .borrow_mut()
            .get_with_both_keys(&Oder::new_with_kuh(Kuh::Borrowed(krate_name), Kuh::Owned(krate)))
        {
            cr.add_dependency(dependency, &dependency_name);
        } else {
            todo!()
        }
    }

    pub fn search(&self, krate: &String) -> Option<UnrolledCrate<'a>> {
        let root = self
            .map
            .borrow()
            .get_with_base_key(Kuh::Borrowed(krate))
            .cloned();
        dbg!(root.map(|r| self.generate_from_crate(&r)))
    }

    pub fn generate_from_crate(&self, krate: &Crate) -> UnrolledCrate<'a> {
        UnrolledCrate {
            crate_id: Kuh::Owned(krate.krate.id),
            name: Kuh::Borrowed(krate.krate.name),
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(Oder { left, right }, _)| {
                    left.as_ref()
                        .zip(right.as_ref())
                        .and_then(|(l, r)| self.generate_if_not_traversed(l, r))
                })
                .collect(),
        }
    }

    pub fn generate_from_crate_name(&'a self, krate_name: &str) -> Option<UnrolledCrate<'a>> {
        let map = self.map.borrow();
        let krate = map.get_with_base_key(&krate_name)?;
        Some(UnrolledCrate {
            crate_id: Kuh::Owned(krate.krate.id),
            name: Kuh::Borrowed(&krate.krate.name),
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(Oder { left, right }, _)| {
                    left.as_ref()
                        .zip(right.as_ref())
                        .and_then(|(l, r)| self.generate_if_not_traversed(l, r))
                })
                .collect(),
        })
    }

    pub fn generate_if_not_traversed(
        &self,
        krate: &Kuh<'_, String>,
        crate_id: &Kuh<'_, u32>,
    ) -> Option<UnrolledCrate<'a>> {
        if self
            .traversed
            .borrow()
            .contains_both_keys(krate, crate_id)
        {
            Some(UnrolledCrate {
                crate_id: crate_id.to_owned(),
                name: krate.to_owned(),
                dependents: Vec::default(),
            })
        } else {
            self.generate_from_crate_name(krate.derow())
        }
    }
}
