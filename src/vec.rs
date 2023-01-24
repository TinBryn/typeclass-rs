use crate::prelude::*;

impl<A> Higher for Vec<A> {
    type Item = A;
    type With<T> = Vec<T>;
}

impl<A> FunctorMut for Vec<A> {
    fn fmap_mut<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Self::With<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Pure for Vec<A> {
    fn pure(a: Self::Item) -> Self {
        vec![a]
    }
}
