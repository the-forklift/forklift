use serde::Deserialize;
use std::io::Read;
use csv::Reader;
use chrono::NaiveDateTime;


#[derive(Clone, Debug, Deserialize)]
pub struct Kiste {
    created_at: String,
    description: String,
    documentation: String,
    homepage: String,
    id: u32,
    max_features: String,
    max_upload_size: Option<u32>,
    name: String,
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
