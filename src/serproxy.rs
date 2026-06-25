use crate::carriage::Carriage;
use crate::store::{Crate, Kiste, Skid};
use serde::{Deserialize, Deserializer, de::Error, de::SeqAccess, de::Visitor};
use serde::{Serialize, Serializer, ser::SerializeMap, ser::SerializeStruct};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::rc::Rc;

pub struct CarriageSer {
    pub map: Rc<RefCell<BTreeMap<u32, CrateSer>>>,
}

impl CarriageSer {
    pub fn from_carriage(x: &Carriage) -> Self {
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

    pub fn insert(&self, key: u32, value: CrateSer) {
        self.map.borrow_mut().insert(key, value);
    }
}

impl Serialize for CarriageSer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.map.borrow().len();
        let mut map = serializer.serialize_map(Some(len))?;
        self.map.borrow().iter().for_each(|(k, v)| {
            let _ = map.serialize_entry(&k, &v);
        });
        map.end()
    }
}

impl<'de> Deserialize<'de> for CarriageSer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = BTreeMap::deserialize(deserializer)?;
        Ok(CarriageSer {
            map: Rc::new(RefCell::new(map)),
        })
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
        let mut state = serializer.serialize_struct("CrateSer", 2)?;
        state.serialize_field("krate", &self.krate)?;
        state.serialize_field("dependencies", &*self.dependencies.borrow())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for CrateSer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("CrateSer", &["krate", "dependencies"], CrateSerVisitor)
    }
}

impl From<CarriageSer> for Carriage {
    fn from(x: CarriageSer) -> Self {
        todo!()
    }
}

struct CrateSerVisitor;

impl<'de> Visitor<'de> for CrateSerVisitor {
    type Value = CrateSer;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "expecting a crate to be delivered")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let krate = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;
        let dependencies = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;

        Ok(CrateSer {
            krate,
            dependencies: Rc::new(RefCell::new(dependencies)),
        })
    }
}
