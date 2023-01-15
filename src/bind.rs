use crate::prelude::{Higher, HigherImpl, HigherKind};

pub trait BindOnceImpl: BindMutImpl {
    fn bind_once<A, B, F: FnOnce(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait BindMutImpl: BindImpl {
    fn bind_mut<A, B, F: FnMut(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait BindImpl: HigherImpl {
    fn bind<A, B, F: Fn(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

impl<T: BindOnceImpl> BindMutImpl for T {
    fn bind_mut<A, B, F: FnMut(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::bind_once(fa, f)
    }
}

impl<T: BindMutImpl> BindImpl for T {
    fn bind<A, B, F: Fn(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::bind_mut(fa, f)
    }
}

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
