use crate::carriage::Carriage;
use crate::fs::Mast;
use anyhow::Error;
use anyhow::Result;
use std::path::Path;

pub trait Crusher: Sized {
    type Floam;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::Floam) -> Result<()>;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}

impl Crusher for Mast {
    type Floam = Carriage;
    fn crush<P: AsRef<Path>>(&self, file: P, contents: &Self::Floam) -> Result<()> {
        let contents = toml::to_string(contents)?;
        std::fs::write(file, contents).map_err(Error::msg)
    }
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        let contents = String::from_utf8(contents)?;
        toml::from_str(&contents).map_err(Error::msg)
    }

}
