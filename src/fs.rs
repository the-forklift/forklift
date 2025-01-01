use crate::store::{Crate, Depencil, Kiste, Lesart};
use csv::Reader;
use flate2::read::GzDecoder;
use sicht::{selector::Oder, SichtMap};
use std::ptr::NonNull;
use std::fs::File;
use std::path::Path;
use tar::Archive;

#[derive(Debug)]
pub struct Carriage {
    pub map: SichtMap<String, u32, Crate>,
    unresolved: Vec<u32>,
}

impl Carriage{
    pub fn new(map: SichtMap<String, u32, Crate>) -> Self {
        Self {
            map,
            unresolved: Vec::default(),
        }
    }

    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let (carriage, stored_entry) = archive.entries().unwrap().fold(
            (Option::<Carriage>::None, None),
            |(mut carriage, mut stored_entry), entry| {
                if let Ok(entry) = entry
                    && let Ok(path) = entry.path()
                    && let Some(csv) = path.extension().and_then(|x| x.to_str())
                {
                    match path {
                        p if p.ends_with("crates.csv") => {
                            let kisten = Reader::from_reader(entry)
                                .deserialize::<Kiste>()
                                .flat_map(|cr| {
                                    cr.map(|c| {
                                        (Oder::new(c.name.clone(), c.id), Crate::new(c))
                                    })
                                })
                                .collect();

                            carriage = Some(Carriage::new(kisten));
                        }
                        p if p.ends_with("dependencies.csv")
                            && let Some(ref mut carr) = carriage =>
                        {
                            Reader::from_reader(entry)
                                .deserialize::<Depencil>()
                                .for_each(|dep| {
                                    if let Ok(ref d) = dep {
                                        carr.add_dependency(d.to_owned());
                                    }
                                });
                        }
                        p if p.ends_with("versions.csv")
                            && let Some(ref mut carr) =  carriage  =>
                        {
                            stored_entry = Some(entry);
                        },
                        _ => {}
                    }
                }

                (carriage, stored_entry)
            },
        );
        if let Some(mut carriage) = carriage {
            Reader::from_reader(stored_entry.unwrap())
                .deserialize::<Lesart>()
                .for_each(|ver| {
                    if let Ok(v) = ver {
                        carriage.add_version(&v);
                    }
                });
            Ok(carriage)
        } else {
            todo!()
        }
    }

    pub fn add_dependency(&mut self, dependency: Depencil) {
        let dep = {
            let Some(r) = self.resolve_dependency(&dependency) else { return; };

            NonNull::new(r as *const _ as *mut Crate).unwrap()
        };
        if let Some(cr) = self.map.get_with_outer_key_mut(&dependency.crate_id) {
            cr.add_dependency(dependency.id, dep);
        } else {

            todo!()
        }
    }

    pub fn resolve_dependency(&mut self, dependency: &Depencil) -> Option<&Crate>{
        if let Some(k) = self.map.get_with_outer_key_mut(&dependency.crate_id) {
            Some(k)
        } else {
            self.unresolved.push(dependency.crate_id);
            None
        }
    }


    pub fn add_version(&mut self, version: &Lesart) {}
}
