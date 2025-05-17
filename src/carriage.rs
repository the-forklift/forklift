use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use sicht::selector::Oder;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Carriage<'a> {
    pub map: SichtCell<'a, String, u32, Crate<'a>>,
    pub traversed: SichtCell<'a, String, u32, Skid>,
}

impl<'a> Carriage<'a> {
    pub fn new(map: SichtCell<'a, String, u32, Crate<'a>>) -> Self {
        Self {
            map,
            traversed: SichtCell::default(),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let (carriage, _) = archive.entries().unwrap().fold(
            (Option::<Carriage<'a>>::None, Lookup::default()),
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
                    (
                        Oder::new(Cow::Owned(c.name.to_owned()), Cow::Owned(c.id)),
                        Crate::new(c.to_owned()),
                    )
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
        &'a self,
        krate: u32,
        krate_name: &str,
        dependency: u32,
        dependency_name: String,
    ) {
        if let Some(cr) = self
            .map
            .borrow_mut()
            .get_with_both_keys(&Oder::new(Cow::Borrowed(krate_name), Cow::Owned(krate)))
        {
            cr.add_dependency(dependency, &dependency_name);
        } else {
            todo!()
        }
    }

    pub fn search(&self, krate: &String) -> Option<UnrolledCrate> {
        let root = self
            .map
            .borrow()
            .get_with_base_key(Cow::Borrowed(krate))
            .cloned();
        dbg!(root.map(|r| self.generate_from_crate(&r)))
    }

    pub fn generate_from_crate(&self, krate: &Crate<'a>) -> UnrolledCrate {
        UnrolledCrate {
            crate_id: krate.krate.id,
            name: krate.krate.name.clone(),
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(Oder { left, right }, _)| {
                    left.as_ref()
                        .zip(right.as_ref())
                        .and_then(|(l, r)| self.generate_if_not_traversed(l, *r.as_ref()))
                })
                .collect(),
        }
    }

    pub fn generate_from_crate_name(&'a self, krate_name: &String) -> Option<UnrolledCrate> {
        let map = self.map.borrow();
        let krate = map.get_with_base_key(Cow::Borrowed(&krate_name))?;
        Some(UnrolledCrate {
            crate_id: krate.krate.id,
            name: krate.krate.name.clone(),
            dependents: krate
                .dependencies
                .borrow()
                .iter()
                .filter_map(|(Oder { left, right }, _)| {
                    left.as_ref()
                        .zip(right.as_ref())
                        .and_then(|(l, r)| self.generate_if_not_traversed(l, *r.as_ref()))
                })
                .collect(),
        })
    }

    pub fn generate_if_not_traversed(
        &self,
        krate: &String,
        crate_id: u32,
    ) -> Option<UnrolledCrate> {
        if self
            .traversed
            .borrow()
            .contains_both_keys(Cow::Borrowed(krate), Cow::Owned(crate_id))
        {
            Some(UnrolledCrate {
                crate_id,
                name: krate.to_owned(),
                dependents: Vec::default(),
            })
        } else {
            self.generate_from_crate_name(krate)
        }
    }
}
