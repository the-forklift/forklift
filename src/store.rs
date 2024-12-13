use chrono::NaiveDateTime;
use serde::Deserialize;
use sicht::SichtMap;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Default)]
pub struct Crate {
    krate: Kiste,
    pub dependencies: SichtMap<String, u32, Skid>,
}

impl Crate {
    pub fn new(krate: Kiste) -> Self {
        Self {
            krate,
            dependencies: SichtMap::default(),
        }
    }

    pub fn new_as_cell(krate: Kiste) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(krate)))
    }

    pub fn add_dependency(&mut self, key: u32, dependency: Rc<RefCell<Crate>>) {
        self.dependencies
            .insert_with_cokey(key, Skid::new_with_dependency(dependency));
    }
}

pub enum SchemaElements {
    Kiste(Kiste),
    Depencil(Depencil),
    Lesart(Lesart),
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Kiste {
    created_at: String,
    description: String,
    documentation: String,
    homepage: String,
    pub id: u32,
    max_features: String,
    max_upload_size: Option<u32>,
    pub name: String,
    readme: String,
    repository: String,
    updated_at: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Depencil {
    pub crate_id: u32,
    default_features: bool,
    explicit_name: String,
    features: Vec<String>,
    pub id: u32,
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
    yanked: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Skid {
    dependency: Rc<RefCell<Crate>>,
    version: Option<String>,
}

impl Skid {
    pub fn new(dependency: Rc<RefCell<Crate>>, version: String) -> Self {
        Self {
            dependency,
            version: Some(version),
        }
    }

    pub fn new_with_dependency(dependency: Rc<RefCell<Crate>>) -> Self {
        Self {
            dependency,
            version: None,
        }
    }
}
