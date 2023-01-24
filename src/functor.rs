use crate::prelude::Higher;

pub trait Functor: Higher {
    fn fmap<B, F: Fn(Self::Item) -> B>(self, f: F) -> Self::With<B>;
}

pub trait FunctorMut: Functor {
    fn fmap_mut<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Self::With<B>;
}

pub trait FunctorOnce: FunctorMut {
    fn fmap_once<B, F: FnOnce(Self::Item) -> B>(self, f: F) -> Self::With<B>;
}

impl<T: FunctorMut> Functor for T {
    fn fmap<B, F: Fn(Self::Item) -> B>(self, f: F) -> Self::With<B> {
        self.fmap_mut(f)
    }
}

impl<T: FunctorOnce> FunctorMut for T {
    fn fmap_mut<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Self::With<B> {
        self.fmap_once(f)
    }
}
