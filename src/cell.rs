use kuh::Derow;
use serde::{Deserialize, Deserializer};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct SichtCell<T>(Rc<RefCell<T>>);

impl<T> SichtCell<T> {
    pub fn new(map: T) -> Self {
        Self(Rc::new(RefCell::new(map)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

impl<T> Debug for SichtCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Default for SichtCell<T>
where
    T: Default,
{
    fn default() -> Self {
        SichtCell(Rc::default())
    }
}

impl<'de, T> Deserialize<'de> for SichtCell<T>
where
    T: Deserialize<'de> + 'de,
    Self: 'de,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = T::deserialize(deserializer)?;
        Ok(Self::new(map))
    }
}
