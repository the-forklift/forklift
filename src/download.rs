use crate::fs::Carriage;
use crate::fs::Mast;
use crate::joystick::Query;
use crate::store::Crate;
use anyhow::Result;

pub struct Ignition {
    query: Query,
}

pub struct Engine {
    query: Query,
    carriage: Carriage,
}

impl Ignition {
    pub fn init(query: Query) -> Result<Engine> {
        let carriage = Mast::load("db-dump.tar.gz")?;
        Ok(Engine::new(query, carriage))
    }
}

impl Engine {
    pub fn new(query: Query, carriage: Carriage) -> Self {
        Engine { query, carriage }
    }

    pub fn run(&mut self) -> Result<Option<Crate>> {
        self.query.apply_to_carriage(&mut self.carriage)
    }

    pub fn process_output(&self) -> Result<()> {
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
