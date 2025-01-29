use crate::fs::Carriage;
use crate::joystick::Query;
use crate::store::Crate;
use anyhow::Result;

pub struct Engine {
    query: Query,
}

impl Engine {
    pub fn new(query: Query) -> Self {
        Engine { query }
    }

    pub fn run(&self) -> Result<Crate> {
        let carriage = Carriage::unarchive("db-dump.tar.gz")?;
        todo!()
    }

    pub fn process_output(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn engine_ignites() {
        let engine = Engine {
            query: Query::default(),
        };

        dbg!(engine.get());
    }
}
