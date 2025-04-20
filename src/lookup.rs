use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Lookup {
    dependency: BTreeMap<u32, u32>,
    krate: BTreeMap<u32, String>,
    dependency_version: BTreeMap<u32, u32>,
}

impl Lookup {
    pub fn with_krate(krate: BTreeMap<u32, String>) -> Self {
        Self {
            dependency: BTreeMap::default(),
            krate,
            dependency_version: BTreeMap::default(),
        }
    }

    pub fn insert_dependency(&mut self, crate_id: u32, dependency: u32) {
        self.dependency.insert(crate_id, dependency);
    }

    pub fn get_dependency(&self, crate_id: u32) -> Option<&u32> {
        self.dependency.get(&crate_id)
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    pub fn get_crate_name(&self, crate_id: u32) -> Option<&str> {
        self.krate.get(&crate_id).map(|x| x.as_ref())
    }

    pub fn insert_dependency_relation(&mut self, crate_id: u32, version_id: u32) {
        self.dependency_version.insert(version_id, crate_id);
    }

    pub fn get_dependency_relation_for_version(&self, version_id: u32) -> Option<&u32> {
        self.dependency_version.get(&version_id)
    }
}
