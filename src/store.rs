use serde::Deserialize;
use std::io::Read;
use csv::Reader;
use chrono::NaiveDateTime;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
pub enum CDV { 
    Crates,
    Dependencies,
    Versions
}


#[derive(Debug)]
pub struct Crate {
    krate: Kiste,
    dependencies: BTreeSet<Crate>,
}

impl Crate {
    pub fn new(krate: Kiste) -> Self {
        Self {
            krate,
            dependencies: BTreeSet::default()
        }
    }


}


impl CDV {
    pub fn handle(&self, entry: impl Read)-> Vec<Crate>{
        match self {
            Self::Crates => {
                
                
                
                
            },
            Self::Dependencies => {
                let dependencies: Vec<Depencil> = Reader::from_reader(entry).deserialize().flat_map(|x| x).collect();
            },
            Self::Versions => {
                let versions: Vec<Lesart> = Reader::from_reader(entry).deserialize().flat_map(|x| x).collect();
            },
        }

        todo!()
    }
}

pub enum SchemaElements {
    Kiste(Kiste),
    Depencil(Depencil),
    Lesart(Lesart)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Kiste {
    created_at: String,
    description: String,
    documentation: String,
    homepage: String,
    id: u32,
    max_features: String,
    max_upload_size: Option<u32>,
    pub name: String,
    readme: String,
    repository: String,
    updated_at: String
}

#[derive(Debug, Default, Deserialize)]
pub struct Depencil {
    crate_id: u32,
    default_features: bool,
    explicit_name: String,
    features: Vec<String>,
    id: u32,
    kind: u32,
    optional: bool,
    req: String,
    target: String,
    version_id: u32,
}

#[derive(Debug, Default, Deserialize)]
pub struct Lesart {
    bin_names: Vec<String>, 
    checksum: String,
    crate_id: u32,
    crate_size: u32,
    created_at: NaiveDateTime,
    downloads: u32,
    features: Vec<String>,
    has_lib: bool,
    id: u32,
    license: String,
    links: String,
    num: String,
    published_by: NaiveDateTime,
    rust_version: String,
    updated_at: NaiveDateTime,
    yanked: bool
}
