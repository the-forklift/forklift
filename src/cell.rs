use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sicht::SichtMap;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub struct SichtCell<K, O, V>(Rc<RefCell<SichtMap<K, O, V>>>)
where
    K: Ord,
    O: Ord;

impl<K, O, V> SichtCell<K, O, V>
where
    K: Ord,
    O: Ord,
{
    pub fn new(map: SichtMap<K, O, V>) -> Self {
        Self(Rc::new(RefCell::new(map)))
    }

    pub fn borrow(&self) -> Ref<'_, SichtMap<K, O, V>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, SichtMap<K, O, V>> {
        self.0.borrow_mut()
    }
}

impl<K, O, V> Clone for SichtCell<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Clone,
    Rc<RefCell<SichtMap<K, O, V>>>: Clone,
{
    fn clone(&self) -> Self {
        SichtCell(Rc::new(RefCell::new(self.0.borrow_mut().clone())))
    }
}

impl<K, O, V> Debug for SichtCell<K, O, V>
where
    K: Ord + Debug,
    O: Ord + Debug,
    V: Debug,
    Rc<RefCell<SichtMap<K, O, V>>>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, O, V> Default for SichtCell<K, O, V>
where
    K: Ord + Default,
    O: Ord + Default,
    Rc<RefCell<SichtMap<K, O, V>>>: Default,
{
    fn default() -> Self {
        SichtCell(Rc::default())
    }
}

impl<K, O, V> Serialize for SichtCell<K, O, V>
where
    K: Ord + Serialize,
    O: Ord + Serialize,
    SichtMap<K, O, V>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de, K, O, V> Deserialize<'de> for SichtCell<K, O, V>
where
    K: Ord + Deserialize<'de> + Debug,
    O: Ord + Deserialize<'de> + Debug,
    V: Debug,
    SichtMap<K, O, V>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = SichtMap::deserialize(deserializer)?;
        Ok(Self::new(map))
    }
}
