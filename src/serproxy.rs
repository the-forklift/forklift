use crate::carriage::Carriage;
use crate::cell::SichtCell;
use crate::store::Crate;
use serde::{Deserialize, Deserializer, de::Error, de::SeqAccess, de::Visitor};
use serde::{Serialize, Serializer, ser::SerializeMap, ser::SerializeStruct};
use sicht::SichtMap;
use std::fmt::Formatter;

impl Serialize for Carriage {
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

impl<'de> Deserialize<'de> for Carriage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = SichtMap::deserialize(deserializer)?;
        Ok(Carriage {
            map: SichtCell::new(map),
            traversed: SichtCell::default(),
        })
    }
}

impl Serialize for Crate {
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

impl<'de> Deserialize<'de> for Crate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Crate", &["krate", "dependencies"], CrateVisitor)
    }
}

struct CrateVisitor;

impl<'de> Visitor<'de> for CrateVisitor {
    type Value = Crate;

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

        Ok(Crate {
            krate,
            dependencies: SichtCell::new(dependencies),
        })
    }
}
