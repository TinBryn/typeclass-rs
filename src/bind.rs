use crate::higher::HigherImpl;

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
