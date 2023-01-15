use crate::prelude::{Higher, HigherKind};

mod implementation;
pub use implementation::{BindImpl, BindMutImpl, BindOnceImpl};

pub trait Bind: Higher {
    fn bind<B, F: Fn(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindMut: Bind {
    fn bind_mut<B, F: FnMut(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindOnce: BindMut {
    fn bind_once<B, F: FnOnce(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

impl<T: HigherKind> Bind for T
where
    Self::Impl: BindImpl,
{
    fn bind<B, F: Fn(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind::<Self::Item, B, F>(self, f)
    }
}

impl<T: HigherKind> BindMut for T
where
    Self::Impl: BindMutImpl,
{
    fn bind_mut<B, F: FnMut(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind_mut::<Self::Item, B, F>(self, f)
    }
}

impl<T: HigherKind> BindOnce for T
where
    Self::Impl: BindOnceImpl,
{
    fn bind_once<B, F: FnOnce(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind_once::<Self::Item, B, F>(self, f)
    }
}
