use std::ops::Deref;

pub struct NewType<T>(T);

impl<T> NewType<T> {
    pub fn new(v: T) -> NewType<T> {
        NewType(v)
    }

    pub fn unbox(self) -> T {
        self.0
    }
}

impl<T> Deref for NewType<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
