use crate::prelude::Higher;

pub trait Apply: Higher {
    fn apply<B, F: Fn(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyMut: Apply {
    fn apply_mut<B, F: FnMut(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyOnce: ApplyMut {
    fn apply_once<B, F: FnOnce(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

impl<T: ApplyOnce> ApplyMut for T {
    fn apply_mut<B, F: FnMut(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        self.apply_once(ff)
    }
}

impl<T: ApplyMut> Apply for T {
    fn apply<B, F: Fn(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        self.apply_mut(ff)
    }
}
