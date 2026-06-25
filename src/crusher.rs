use crate::carriage::Carriage;
use crate::fs::Mast;
use crate::serproxy::CarriageSer;
use anyhow::Error;
use anyhow::Result;
use std::fs::OpenOptions;
use std::path::Path;

pub trait Crusher: Sized {
    type Floam;
    type FloamSer = Self::Floam;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::FloamSer) -> Result<()>;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}

impl Crusher for Mast {
    type Floam = CarriageSer;
    type FloamSer = CarriageSer;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::FloamSer) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)?;
        ron::ser::to_writer(file, &contents)?;

        Ok(())
    }
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        (ron::de::from_bytes(&contents)).map_err(Error::msg)
    }
}
