use crate::fs::Carriage;
use crate::fs::Mast;
use crate::joystick::Query;
use crate::store::Crate;
use anyhow::Result;

pub struct Ignition {
    query: Query,
    config: Config,
}

pub struct Engine {
    query: Query,
    carriage: Carriage,
}

impl Ignition {
    pub fn init(query: Query) -> Result<Engine> {
        let carriage = Mast::path("db-dump.tar.gz").load()?;
        Ok(Engine::new(query, carriage))
    }

    pub fn init_with_config(query: Query, config: Config) -> Result<Engine> {
        let carriage = Mast::path("db-dump.tar.gz").config(config).load()?;
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

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub fresh: bool,
}

impl Config {
    pub fn fresh() -> Self {
        let mut config = Config::default();
        config.fresh = true;
        config
    }
}
