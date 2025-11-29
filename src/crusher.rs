use crate::carriage::Carriage;
use crate::fs::Mast;
use anyhow::Result;

pub trait Crusher: Sized {
    type Floam;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}

impl Crusher for Mast {
    type Floam = Carriage;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        ron::de::from_bytes(&contents).map_err(|_e| todo!())
    }
}
