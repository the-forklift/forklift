use serde::{Deserialize, Serialize};
use sicht::SichtMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn add_dependency(&mut self, key: u32, krate_name: String) {
        self.dependencies
            .insert_with_both_keys(krate_name, key, Skid::new_with_dependency(key));
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Kiste {
    pub created_at: String,
    description: String,
    homepage: String,
    pub id: u32,
    max_features: String,
    max_upload_size: Option<u32>,
    pub name: String,
    repository: String,
    updated_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Depencil {
    pub crate_id: u32,
    default_features: Option<String>,
    explicit_name: Option<String>,
    features: Option<String>,
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
    #[serde(default)]
    pub crate_id: Option<u32>,
    crate_size: Option<u32>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skid {
    dependency: u32,
    version: Option<String>,
}

impl Skid {
    pub fn new(dependency: u32, version: String) -> Self {
        Self {
            dependency,
            version: Some(version),
        }
    }

    pub fn new_with_dependency(dependency: u32) -> Self {
        Self {
            dependency,
            version: None,
        }
    }
}
