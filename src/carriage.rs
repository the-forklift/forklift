use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Cdv, Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::Error;
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use serde::Deserialize;
use sicht::SichtMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::Path;
use std::{fs::File, io::Read};
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

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
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
                        .map(|dep| (dep.crate_id, dep.version_id))
                        .collect::<BTreeMap<u32, u32>>();
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

    pub fn process_versions(&self, versions: BTreeMap<u32, u32>) {
        self.lookup.borrow_mut().seed_dependencies(versions);
    }

    pub fn process_dependencies(&self, dependencies: BTreeMap<u32, u32>) {
        let lookup = self.lookup.borrow();
        dependencies
            .into_iter()
            .filter_map(|(crate_id, version_id)| {
                lookup
                    .get_dependency_relation_for_version(version_id)
                    .map(|x| (crate_id, x))
            })
            .for_each(|(crate_id, dependency)| {
                if let Some(dependency_name) = lookup.get_crate_name(*dependency) {
                    self.add_dependency(crate_id, *dependency, dependency_name);
                }
            });
    }

    pub fn add_dependency(&'a self, krate: u32, dependency: u32, dependency_name: &str) {
        if let Some(cr) = self.map.borrow_mut().get(&krate) {
            cr.add_dependency(dependency, dependency_name);
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

/*
impl<'de> Deserialize<'de> for Carriage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CarriageVisitor<K, O, V>
        where
            K: Clone + Ord,
            O: Clone + Ord,
        {
            map: SichtMap<K, O, V>,
            traversed: (),
            lookup: SichtMap<K, O, V>,
        }

        impl<K, O, V> CarriageVisitor<K, O, V>
        where
            K: Clone + Ord,
            O: Clone + Ord,
        {
            fn new() -> Self {
                Self {
                    map: SichtMap::default(),
                    traversed: (),
                    lookup: SichtMap::default(),
                }
            }
        }

        impl<'df, K, O, V> Visitor<'df> for CarriageVisitor<K, O, V>
        where
            K: Clone + Ord,
            O: Clone + Ord,
        {
            type Value = Carriage;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder are malformed")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'df>,
            {
                self.map = SichtMap::deserialize(map);
            }
        }

        deserializer.deserialize_struct(
            "Carriage",
            &["map", "traversed", "lookup"],
            CarriageVisitor::<u32, String, Crate>::new(),
        )
    }
}
*/
