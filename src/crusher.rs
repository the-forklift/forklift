use crate::carriage::Carriage;
use crate::error::Error;
use crate::fs::Mast;
use std::path::Path;

pub trait Crusher: Sized {
    type Floam;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::Floam) -> Result<(), Error>;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam, Error>;
}

impl Crusher for Mast {
    type Floam = Carriage;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::Floam) -> Result<(), Error> {
        let contents = toml::to_string(contents).map_err(|_| Error {})?;
        std::fs::write(file, contents).map_err(|_| Error {})
    }
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam, Error> {
        toml::from_slice(&contents).map_err(|_| Error {})
    }
}
