use crate::higher::HigherImpl;

pub trait ApplyImpl: HigherImpl {
    fn apply<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

pub trait ApplyMutImpl: ApplyImpl {
    fn apply_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

pub trait ApplyOnceImpl: ApplyMutImpl {
    fn apply_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

impl<T: ApplyOnceImpl> ApplyMutImpl for T {
    fn apply_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        Self::apply_once(fa, ff)
    }
}

impl<T: ApplyMutImpl> ApplyImpl for T {
    fn apply<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        Self::apply_mut(fa, ff)
    }
}
