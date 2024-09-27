use std::io::Read;
use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::store::{Crate, Kiste, Depencil, Lesart} ;
use std::collections::BTreeMap;
use csv::Reader;

#[derive(Debug, Default)]
pub struct Carriage(BTreeMap<String, Crate>);

#[derive(Debug, Default)]
pub struct Feil {
    dependencies: Vec<u8>, 
    crates:  Vec<u8>,
    versions: Vec<u8>
}

impl Feil {

    pub fn new(dependencies: Vec<u8>, crates: Vec<u8>, versions: Vec<u8>) -> Self {
        Self {
            dependencies,
            crates,
            versions,
        }
    }
}

pub struct Mapper {
    
}

impl Carriage { 
    pub fn unarchive<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(GzDecoder::new(file));
        let carriage = archive.entries().unwrap().fold(Option::<Carriage>::None, |mut carriage, entry| {
        if let Ok(entry) = entry && let Ok(path) = entry.path() && let Some(csv) = path.extension().and_then(|x| x.to_str()) {
            match path {
                p if p.ends_with("crates.csv") => {
                    let kisten = Reader::from_reader(entry).deserialize::<Kiste>().flat_map(|cr| {
                        cr.map(|c| (c.name.clone(), Crate::new(c)))
                    }).collect();

                    carriage = Some(Carriage(kisten));
                    
                },
                p if p.ends_with("dependencies.csv") && let Some(ref mut carr) = carriage => {
                    Reader::from_reader(entry).deserialize::<Depencil>().for_each(|dep| {
                        if let Ok(d) = dep {
                            carr.add_dependency(d);
                        } 
                    });
                },
                p if p.ends_with("versions.csv") && let Some(ref mut carr) = carriage => {
                    Reader::from_reader(entry).deserialize::<Lesart>().for_each(|ver| {
                        if let Ok(v) = ver {
                            carr.add_version(v);
                        }
                    });
                },
                _ => todo!(),
                } 
        }

        carriage
        });
        todo!()
    }

    pub fn add_dependency(&mut self, dependency: Depencil) {
        self.0.entry(
    }

    pub fn add_version(&mut self, version: Lesart) {
    }

}


pub struct Feiled <J: Jumper> {
    file: File,
    jumper: J,
    contents: Vec<u8>,
}

impl<J> Feiled<J> 
where
    J: Jumper + Default,
{
    pub fn open<D, P>(path: D, jumper: J) -> Result<Self, anyhow::Error> 
        where
            D: AsRef<Path>,
            P: Preprocessor,
    {
        let mut file = File::open(path)?;
        let contents = P::new(GzDecoder::new(file)); 

        todo!()
    }


    pub fn erskip_till<const N: usize>(&mut self, krate: &str, buffer: &mut [u8; N]) -> Result<(), anyhow::Error> {
       
        let mut slices = self.contents.iter().take(128).enumerate().fold((Vec::new(), Vec::<u8>::new(), 0u16, 0usize, None), |(mut krate, mut dependency, mut dep_count, mut offset, mut hdc), (i, byte)| {
            match dbg!(i - offset, dep_count) {
                (0..=31, _) => {
                    krate.push(byte);
                },
                (32, 0) => {
                    hdc = Some(byte);
                },
                (33, 0) if let Some(half) = hdc => {
                    dep_count = process_count_from_bytes(*half, *byte);
                },

                (n, dc) if n - dc as usize == 17 => { 
                    offset = n;
                    dep_count = 0;
                    hdc = None;
                    dependency.push(*byte);
                },
                (n, dc) => {
                    dependency.push(*byte);
                }
            }
            (krate, dependency, dep_count, offset, hdc)
        });
        dbg!(&slices);


        todo!()
    }
            

    pub fn wskip_till<const N: usize>(&mut self, krate: &str, buffer: &mut [u8; N]) -> Result<(), anyhow::Error> {
        loop {
            let _ = self.file.read_exact(&mut buffer[..])?;
            if self.jumper.till_condition(krate, buffer) {
                return Ok(());
            }
            let mut count_buf = [0; 8];
            self.jumper.skip_condition(&mut self.file, &mut count_buf); 
        } 
    }

}
 

impl<J: Jumper> Read for Feiled<J> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buffer)
    }
}

pub trait Jumper {
    fn till_condition<const N: usize>(&self, krate: &str, buffer: &mut [u8; N] ) -> bool;
    fn skip_condition<const N: usize>(&self, file: &mut File, buffer: &mut [u8; N]) -> bool;
}

pub trait Unarchiver {
    fn open(reader: impl Read) -> Self;
}
pub trait Preprocessor {
    fn new(reader: impl Read) -> Self;
    fn preprocess(&self) -> Vec<u8>;
}

pub fn process_count_from_bytes(half: u8, rest: u8) -> u16 {
    dbg!(255 - &half, 255 - &rest);
    let word = String::from_utf8(vec![half]).unwrap();
    dbg!(&word);
    word.parse().unwrap()
}
