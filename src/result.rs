use crate::prelude::{FunctorOnce, Higher};

impl<A, E> Higher for Result<A, E> {
    type Item = A;

    type With<T> = Result<T, E>;
}

impl<A, E> FunctorOnce for Result<A, E> {
    fn fmap_once<B, F: FnOnce(Self::Item) -> B>(self, f: F) -> Self::With<B> {
        self.map(f)
    }
}
