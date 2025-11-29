use crate::carriage::Carriage;
use crate::store::{Crate, Kiste, Skid};
use serde::{Serialize, Serializer, ser::SerializeMap};
use sicht::selector::Oder;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct CarriageSer {
    pub map: Rc<RefCell<BTreeMap<(String, u32), CrateSer>>>,
}

impl<'a> From<Carriage> for CarriageSer {
    fn from(x: Carriage) -> Self {
        let map = x
            .map
            .borrow()
            .iter()
            .filter_map(|(od, v)| match od {
                Oder {
                    left: Some(k),
                    right: Some(o),
                } => Some(((String::from(k), *o), CrateSer::from(v.clone()))),
                _ => None,
            })
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
        self.map.borrow().iter().for_each(|((k, o), v)| {
            map.serialize_entry(&(k, o), &v);
        });
        map.end()
    }
}

pub struct CrateSer {
    krate: Kiste,
    dependencies: Rc<RefCell<BTreeMap<(String, u32), Skid>>>,
}

impl From<Crate> for CrateSer {
    fn from(x: Crate) -> Self {
        let map = x
            .dependencies
            .borrow()
            .iter()
            .map(|(Oder { left, right }, d)| match (left, right) {
                (Some(l), Some(r)) => (((l).to_owned(), *r), d.to_owned()),
                _ => todo!(),
            })
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
