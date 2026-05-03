use crate::carriage::Carriage;
use crate::fs::Mast;
use anyhow::Error;
use anyhow::Result;

pub trait Crusher: Sized {
    type Floam;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}

impl Crusher for Mast {
    type Floam = Carriage;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        let _ = dbg!(String::from_utf8(contents));
        // let _ = dbg!(String::from_utf8(contents.iter().take(7).map(|x| *x).collect::<Vec<u8>>()));
        todo!()
        // let err = dbg!(ron::de::from_bytes(&contents)).map_err(Error::msg);
    }
}
