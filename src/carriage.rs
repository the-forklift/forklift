use crate::cell::SichtCell;
use crate::lookup::Lookup;
use crate::store::Skid;
use crate::store::{Crate, Depencil, Kiste, Lesart, UnrolledCrate};
use anyhow::Result;
use csv::Reader;
use flate2::read::GzDecoder;
use serde::{Deserialize, Deserializer, de::Visitor};
use std::collections::{BTreeMap, btree_map::IntoIter};
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::{cell::RefCell, rc::Rc};
use std::{fs::File, io::Read};
use tar::Archive;

#[derive(Clone, Debug, Default)]
pub struct Carriage {
    pub map: SichtCell<u32, String, Crate>,
    pub traversed: SichtCell<u32, String, Skid>,
    pub lookup: Rc<RefCell<Lookup>>,
}

impl<'a> Carriage {
    pub fn new(map: SichtCell<u32, String, Crate>, lookup: Lookup) -> Self {
        Self {
            map,
            traversed: SichtCell::default(),
            lookup: Rc::new(RefCell::new(lookup)),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(&'a mut self, path: P) -> Result<(), anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let order = archive
            .entries()
            .unwrap()
            .filter_map(|e| match e {
                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("crates.csv") =>
                {
                    Some((0, e))
                }
                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("versions.csv") =>
                {
                    Some((1, e))
                }
                Ok(e)
                    if let Ok(p) = e.path()
                        && p.ends_with("dependencies.csv") =>
                {
                    Some((2, e))
                }
                _ => None,
            })
            .collect::<BTreeMap<usize, _>>();

        let mut order = order.into_iter();
        let (_, ent) = order.next().unwrap();

        self.process_crates(ent);

        self.process_crate_information(order);

        Ok(())
    }

    fn process_crate_information(&'a mut self, order: IntoIter<usize, impl Read>) {
        order.for_each(|(i, ent)| {
            match i {
                1 => {
                    self.process_dependencies(ent);
                }
                2 => self.process_versions(ent),
                _ => unreachable!(),
            };
        });
    }

    pub fn process_crates(&mut self, entry: impl Read) {
        let map = Reader::from_reader(entry)
            .deserialize::<Kiste>()
            .filter_map(|cr| cr.ok())
            .map(|cr| {
                let name = cr.name.to_owned();
                (cr.id, name, Crate::new(cr.to_owned()))
            })
            .collect();

        self.map = SichtCell::new(map)
    }

    #[allow(clippy::unused_self)]
    pub fn process_versions(&self, entry: impl Read) {
        Reader::from_reader(entry)
            .deserialize::<Lesart>()
            .for_each(|ver| {
                if let Ok(ref v) = ver
                    && let Some(crate_id) = v.crate_id
                {
                    self.lookup
                        .borrow_mut()
                        .insert_dependency_relation(v.id, crate_id);
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
                    let krate_name = self
                        .lookup
                        .borrow()
                        .get_crate_name(d.crate_id)
                        .map(ToOwned::to_owned);
                    let dependency = self
                        .lookup
                        .borrow()
                        .get_dependency_relation_for_version(d.version_id)
                        .copied();
                    if let Some(krate_name) = krate_name
                        && let Some(dependency) = dependency
                        && let Some(dependency_name) =
                            self.lookup.borrow().get_crate_name(dependency)
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
        krate_name: String,
        dependency: u32,
        dependency_name: String,
    ) {
        if let Some(cr) = self.map.borrow_mut().get(&krate) {
            cr.add_dependency(dependency, &dependency_name);
        } else {
            todo!()
        }
    }

    pub fn search(&self, krate: &'a str) -> Option<UnrolledCrate> {
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

    pub fn generate_from_crate_name(&'a self, krate_name: &str) -> Option<UnrolledCrate> {
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

impl<'de> Deserialize<'de> for Carriage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CarriageVisitor {
            map: (),
            lookup: (),
        }

        impl CarriageVisitor {
            fn new() -> Self {
                Self {
                    map: (),
                    lookup: (),
                }
            }
        }

        impl<'de, 'a> Visitor<'de> for CarriageVisitor {
            type Value = Carriage;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_struct("Carriage", &["map", "lookup"], CarriageVisitor::new())
    }
}
