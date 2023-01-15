mod implementation;
pub use implementation::{ApplyImpl, ApplyMutImpl, ApplyOnceImpl};

use crate::prelude::*;

pub trait Apply<A>: Higher<A> {
    fn apply<B, F: Fn(A) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyMut<A>: Apply<A> {
    fn apply_mut<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait ApplyOnce<A>: ApplyMut<A> {
    fn apply_once<B, F: FnOnce(A) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

impl<A, T: HigherKind<A>> Apply<A> for T
where
    Self::Impl: ApplyImpl,
{
    fn apply<B, F: Fn(A) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply::<A, B, F>(self, ff)
    }
}

impl<A, T: HigherKind<A>> ApplyMut<A> for T
where
    Self::Impl: ApplyMutImpl,
{
    fn apply_mut<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply_mut::<A, B, F>(self, ff)
    }
}

impl<A, T: HigherKind<A>> ApplyOnce<A> for T
where
    Self::Impl: ApplyOnceImpl,
{
    fn apply_once<B, F: FnOnce(A) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Self::Impl::apply_once::<A, B, F>(self, ff)
    }
}
