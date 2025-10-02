use crate::carriage::Carriage;
use crate::crusher::Crusher;
use crate::download::Config;
use anyhow::Result;
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

    pub fn load<'a>(&'a mut self) -> Result<Carriage<'a>> {
        if !self.config.fresh
            && let Ok(mut file) = OpenOptions::new().read(true).open("lager.fork")
        {
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer)?;
            Self::uncrush(buffer)
        } else {
            let contents = Carriage::unarchive(&self.path)?;
            let _ = Self::store_contents(&contents);
            Ok(contents)
        }
    }

    pub fn store_contents<'a>(contents: &Carriage<'a>) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("lager.fork")?;
        ron::ser::to_writer(file, &contents)?;
        Ok(())
    }
}
