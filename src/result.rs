use crate::prelude::{ApplyOnceImpl, BindOnceImpl, FunctorOnceImpl, HigherImpl, HigherKind, PointImpl};

// stores the E, it doesn't matter as this type is never meant to be instantiated
pub struct ResultImpl<E>(E);

impl<E> HigherImpl for ResultImpl<E> {
    type Kind<T> = Result<T, E>;
}

impl<T, E> HigherKind<T> for Result<T, E> {
    type Impl = ResultImpl<E>;
}

impl<E> FunctorOnceImpl for ResultImpl<E> {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.map(f)
    }
}

impl<E> PointImpl for ResultImpl<E> {
    fn point<A>(a: A) -> Self::Kind<A> {
        Ok(a)
    }
}

impl<E> ApplyOnceImpl for ResultImpl<E> {
    fn apply_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        fa.and_then(|a| ff.map(|f| f(a)))
    }
}

impl<E> BindOnceImpl for ResultImpl<E> {
    fn bind_once<A, B, F: FnOnce(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.and_then(f)
    }
}
