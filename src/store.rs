use crate::cell::SichtCell;
use kuh::Kuh;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Crate<'a> {
    pub krate: Kiste,
    pub dependencies: SichtCell<'a, String, u32, Skid>,
}
impl<'a> Crate<'a> {
    pub fn new(krate: Kiste) -> Self {
        Self {
            krate,
            dependencies: SichtCell::default(),
        }
    }

    pub fn add_dependency(&self, key: u32, krate_name: &str) {
        self.dependencies.borrow_mut().insert_with_both_keys(
            krate_name.to_owned(),
            key,
            Skid::new_with_dependency(key),
        );
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

#[derive(Clone, Debug, Default)]
pub struct UnrolledCrate<'a> {
    pub crate_id: Kuh<'a, u32>,
    pub name: Kuh<'a, String>,
    pub dependents: Vec<Self>,
}
impl<'a> UnrolledCrate<'a> {
    pub fn new(crate_id: Kuh<'a, u32>, name: Kuh<'a, String>, dependents: Vec<Self>) -> Self {
        Self {
            crate_id,
            name,
            dependents,
        }
    }
}

impl<'de, 'a> Deserialize<'de> for Crate<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CrateVisitor<'a> {
            map: PhantomData<&'a ()>,
            lookup: PhantomData<&'a ()>,
        }

        impl CrateVisitor<'_> {
            fn new() -> Self {
                Self {
                    map: PhantomData,
                    lookup: PhantomData,
                }
            }
        }

        impl<'de, 'a> Visitor<'de> for CrateVisitor<'a> {
            type Value = Crate<'a>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_struct("Crate", &["krate", "dependencies"], CrateVisitor::new())
    }
}
