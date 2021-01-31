use std::marker::PhantomData;

pub mod documents;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Id<T> {
    id: u64,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> u64 {
        self.id
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Id::new(0)
    }
}
