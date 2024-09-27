use anyhow::Result;
use crate::joystick::Query;
use crate::fs::Carriage;
use crate::store::Crate;
use std::{fs::File, io::Read};

pub struct Engine {
    krate: Crate,
    query: Query,
}

impl Engine {

    pub fn get(&self) -> Result<Crate> {
    let mut file = Carriage::unarchive("db-dump.tar.gz")?;
    dbg!(&file);
    todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn engine_ignites() {
        let engine = Engine {
            krate: Crate::new("foo"),
            query: Query::default()
        };

        dbg!(engine.get());

    }
}
