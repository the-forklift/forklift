use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sicht::SichtMap;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use kuh::Derow;

#[derive(Clone)]
pub struct SichtCell<'a, K, O, V>(Rc<RefCell<SichtMap<'a, K, O, V>>>)
where
   K: Ord + Derow<'a> + Clone,
    O: Ord + Derow<'a> + Clone;


impl<'a, K, O, V> SichtCell<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone,
    O: Ord + Derow<'a> + Clone,
{
    pub fn new(map: SichtMap<'a, K, O, V>) -> Self {
        Self(Rc::new(RefCell::new(map)))
    }

    pub fn borrow(&self) -> Ref<'_, SichtMap<'a, K, O, V>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, SichtMap<'a, K, O, V>> {
        self.0.borrow_mut()
    }
}

impl<'a, K, O, V> Debug for SichtCell<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone + Debug,
    O: Ord + Derow<'a> + Clone + Debug,
    V: Debug,
    Rc<RefCell<SichtMap<'a, K, O, V>>>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, K, O, V> Default for SichtCell<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone + Default,
    O: Ord + Derow<'a> + Clone + Default,
    Rc<RefCell<SichtMap<'a, K, O, V>>>: Default,
{
    fn default() -> Self {
        SichtCell(Rc::default())
    }
}

impl<'a, K, O, V> Serialize for SichtCell<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone + Serialize,
    O: Ord + Derow<'a> + Clone + Serialize,
    SichtMap<'a, K, O, V>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de, K, O, V> Deserialize<'de> for SichtCell<'de, K, O, V>
where
    K: Ord + Derow<'de> + Deserialize<'de> + Clone,
    O: Ord + Derow<'de> + Deserialize<'de> + Clone,
    V: Debug,
    SichtMap<'de, K, O, V>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = SichtMap::deserialize(deserializer)?;
        Ok(Self::new(map))
    }
}
