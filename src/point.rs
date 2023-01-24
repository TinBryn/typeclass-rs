use crate::prelude::Higher;

pub trait Pure: Higher {
    fn pure(a: Self::Item) -> Self;
}
