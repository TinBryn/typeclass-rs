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

pub trait Bind<A>: Higher<A> {
    fn bind<B, F: Fn(A) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindMut<A>: Bind<A> {
    fn bind_mut<B, F: FnMut(A) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindOnce<A>: BindMut<A> {
    fn bind_once<B, F: FnOnce(A) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

impl<A, T: HigherKind<A>> Bind<A> for T
where
    Self::Impl: BindImpl,
{
    fn bind<B, F: Fn(A) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind::<A, B, F>(self, f)
    }
}

impl<A, T: HigherKind<A>> BindMut<A> for T
where
    Self::Impl: BindMutImpl,
{
    fn bind_mut<B, F: FnMut(A) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind_mut::<A, B, F>(self, f)
    }
}

impl<A, T: HigherKind<A>> BindOnce<A> for T
where
    Self::Impl: BindOnceImpl,
{
    fn bind_once<B, F: FnOnce(A) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        Self::Impl::bind_once::<A, B, F>(self, f)
    }
}
