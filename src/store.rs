use serde::Deserialize;
use sicht::SichtMap;
use std::ptr::NonNull;
use serde::Deserializer;
#[derive(Debug, Clone, Deserialize)]
pub struct Crate {
    pub krate: Kiste,
    pub dependencies: SichtMap<String, u32, Skid>,
}
impl Crate {
    pub fn new(krate: Kiste) -> Self {
        Self {
            krate,
            dependencies: SichtMap::new(),
        }
    }

    pub fn add_dependency(&mut self, key: u32, dependency: NonNull<Crate>) {
        self.dependencies
            .insert_with_cokey(key, Skid::new_with_dependency(dependency));
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Kiste {
    pub created_at: String,
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

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Depencil {
    pub crate_id: u32,
    default_features: Option<String>,
    explicit_name: Option<String>,
    features: String,
    pub id: u32,
    kind: u32,
    optional: String,
    req: String,
    target: String,
    pub version_id: u32,
}

#[derive(Debug, Default, Deserialize)]
pub struct Lesart {
    bin_names: String,
    checksum: String,
    pub crate_id: u32,
    crate_size: u32,
    created_at: String,
    downloads: u32,
    features: String,
    has_lib: String,
    pub id: u32,
    license: String,
    links: String,
    num: String,
    published_by: String,
    rust_version: String,
    updated_at: String,
    yanked: String,
}

#[derive(Clone, Debug)]
pub struct Skid {
    dependency: NonNull<Crate>,
    version: Option<String>,
}

impl Skid {
    pub fn new(dependency: NonNull<Crate>, version: String) -> Self {
        Self {
            dependency,
            version: Some(version),
        }
    }

    pub fn new_with_dependency(dependency: NonNull<Crate>) -> Self {
        Self {
            dependency,
            version: None,
        }
    }
}


impl<'de> Deserialize<'de> for Skid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where
            D: Deserializer<'de>
    {
        todo!()

    }
}
