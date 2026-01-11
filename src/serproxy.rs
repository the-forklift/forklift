use crate::carriage::Carriage;
use crate::store::{Crate, Kiste, Skid};
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct CarriageSer {
    pub map: Rc<RefCell<BTreeMap<u32, CrateSer>>>,
}

impl<'a> From<Carriage> for CarriageSer {
    fn from(x: Carriage) -> Self {
        let map = x
            .map
            .borrow()
            .iter()
            .map(|(od, v): (&u32, &Crate)| (*od, CrateSer::from(v.clone())))
            .collect();

        Self {
            map: Rc::new(RefCell::new(map)),
        }
    }
}

impl<'a> Serialize for CarriageSer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.map.borrow().len();
        let mut map = serializer.serialize_map(Some(len))?;
        self.map.borrow().iter().for_each(|(k, v)| {
            map.serialize_entry(&k, &v);
        });
        map.end()
    }
}

pub struct CrateSer {
    krate: Kiste,
    dependencies: Rc<RefCell<BTreeMap<u32, Skid>>>,
}

impl From<Crate> for CrateSer {
    fn from(x: Crate) -> Self {
        let map = x
            .dependencies
            .borrow()
            .iter()
            .map(|(k, d)| (*k, d.to_owned()))
            .collect();

        CrateSer {
            krate: x.krate,
            dependencies: Rc::new(RefCell::new(map)),
        }
    }
}

impl Serialize for CrateSer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
