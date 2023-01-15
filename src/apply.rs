mod implementation;
pub use implementation::{ApplyImpl, ApplyMutImpl, ApplyOnceImpl};

use crate::prelude::*;

pub trait Apply: Higher {
    fn apply<B, F: Fn(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyMut: Apply {
    fn apply_mut<B, F: FnMut(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyOnce: ApplyMut {
    fn apply_once<B, F: FnOnce(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

impl<T: HigherKind> Apply for T
where
    Self::Impl: ApplyImpl,
{
    fn apply<B, F: Fn(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply::<Self::Item, B, F>(self, ff)
    }
}

impl<T: HigherKind> ApplyMut for T
where
    Self::Impl: ApplyMutImpl,
{
    fn apply_mut<B, F: FnMut(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply_mut::<Self::Item, B, F>(self, ff)
    }
}

impl<T: HigherKind> ApplyOnce for T
where
    Self::Impl: ApplyOnceImpl,
{
    fn apply_once<B, F: FnOnce(Self::Item) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply_once::<Self::Item, B, F>(self, ff)
    }
}
