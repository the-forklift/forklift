use kuh::Derow;
use serde::{Deserialize, Deserializer};
use sicht::SichtMap;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct SichtCell<K, O, V>(Rc<RefCell<SichtMap<K, O, V>>>)
where
    K: Ord + Clone,
    O: Ord + Clone;

impl<'a, K, O, V> SichtCell<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
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

impl<'a, K, O, V> Debug for SichtCell<K, O, V>
where
    K: Ord + Clone + Debug,
    O: Ord + Clone + Debug,
    V: Debug,
    Rc<RefCell<SichtMap<K, O, V>>>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, K, O, V> Default for SichtCell<K, O, V>
where
    K: Ord + Clone + Default,
    O: Ord + Clone + Default,
{
    fn default() -> Self {
        SichtCell(Rc::default())
    }
}

impl<'de, 'a, K, O, V> Deserialize<'de> for SichtCell<K, O, V>
where
    K: Ord + Derow<'de, Target: Ord> + Deserialize<'de> + Clone + 'de,
    O: Ord + Derow<'de, Target: Ord> + Deserialize<'de> + Clone + 'de,
    V: Debug + 'de,
    SichtMap<K, O, V>: Deserialize<'de> + 'de,
    Self: 'de,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = SichtMap::deserialize(deserializer)?;
        Ok(Self::new(map))
    }
}
