use anyhow::Result;
use crate::fs::{Mast, Carriage};
use std::io::Write;

pub trait Crusher: Sized {
    type Floam; 
    fn crush<W: Write>(&self, writer: &W) -> Result<Vec<u8>>;
    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam>;
}


impl Crusher for Mast {
    type Floam = Carriage;
    fn crush<W: Write>(&self, writer: &W) -> Result<Vec<u8>> {
        todo!()
    }

    fn uncrush(contents: Vec<u8>) -> Result<Self::Floam> {
        ron::de::from_bytes(&contents).map_err(|e| { 
            todo!()
        })

    }
            

}
