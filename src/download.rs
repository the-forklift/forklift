use crate::carriage::Carriage;
use crate::fs::Mast;
use crate::joystick::Query;
use crate::store::UnrolledCrate;
use anyhow::Result;

pub struct Ignition {
    query: Query,
    config: Config,
}

pub struct Engine<'a> {
    query: Query,
    carriage: Carriage<'a>,
}

impl Ignition {
    pub fn init<'a>(query: Query) -> Result<Engine<'a>> {
        let mut mast = Mast::path("db-dump.tar.gz");
        let carriage = mast.load()?;
        Ok(Engine::new(query, carriage))
    }

    pub fn init_with_config<'a>(query: Query, config: Config) -> Result<Engine<'a>> {
        let carriage = Mast::path("db-dump.tar.gz").config(config).load()?;
        Ok(Engine::new(query, carriage))
    }
}

impl<'a> Engine<'a> {
    pub fn new(query: Query, carriage: Carriage<'a>) -> Self {
        Engine { query, carriage }
    }

    pub fn run(&'a mut self) -> Result<Option<UnrolledCrate<'a>>> {
        self.query.apply_to_carriage(&mut self.carriage)
    }

    pub fn process_output(&self) -> Result<()> {
        todo!("next stage")
    }
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub fresh: bool,
}

impl Config {
    #[allow(clippy::needless_update)]
    pub fn fresh() -> Self {
        Config {
            fresh: true,
            ..Default::default()
        }
    }
}
