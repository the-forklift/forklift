use crate::carriage::Carriage;
use crate::fs::Mast;
use anyhow::Result;

pub trait Crusher<'a>: Sized {
    type Floam: 'a;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}

impl<'a> Crusher<'a> for Mast {
    type Floam = Carriage<'a>;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        ron::de::from_bytes(&contents).map_err(|e| todo!())
    }
}
