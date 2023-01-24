use crate::prelude::{ApplyOnce, BindOnce, FunctorOnce, Higher, Pure};

impl<A> Higher for Option<A> {
    type Item = A;
    type With<T> = Option<T>;
}

impl<A> FunctorOnce for Option<A> {
    fn fmap_once<B, F: FnOnce(Self::Item) -> B>(self, f: F) -> Self::With<B> {
        self.map(f)
    }
}

impl<A> Pure for Option<A> {
    fn pure(a: Self::Item) -> Self {
        Some(a)
    }
}

impl<A> ApplyOnce for Option<A> {
    fn apply_once<B, F: FnOnce(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        ff.and_then(|f| self.map(f))
    }
}

impl<A> BindOnce for Option<A> {
    fn bind_once<B, F: FnOnce(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        self.and_then(f)
    }
}
