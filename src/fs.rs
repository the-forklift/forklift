use crate::carriage::Carriage;
use crate::crusher::Crusher;
use crate::download::Config;
use crate::error::Error;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Mast {
    path: PathBuf,
    #[serde(skip)]
    config: Config,
}

impl Mast {
    pub fn path<P: AsRef<Path>>(load_path: P) -> Self {
        Mast {
            path: load_path.as_ref().to_owned(),
            config: Config::default(),
        }
    }

    pub fn config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn load(&mut self) -> Result<Carriage, Error> {
        if !self.config.fresh
            && let Ok(mut file) = OpenOptions::new().read(true).open("lager.fork")
        {
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer).map_err(|_| Error {})?;
            Self::uncrush(buffer)
        } else {
            let carriage = Carriage::unarchive(&self.path).map_err(|_| Error {})?;
            let _ = self.store_contents(&carriage);
            Ok(carriage)
        }
    }

    pub fn store_contents(&self, contents: &Carriage) -> Result<(), Error> {
        self.crush("lager.fork", contents)
    }
}
