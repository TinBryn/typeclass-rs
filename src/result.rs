use crate::prelude::*;
pub struct ResultImpl<E>(E);

impl<E> HigherImpl for ResultImpl<E> {
    type Kind<T> = Result<T, E>;
}

impl<T, E> Higher<T> for Result<T, E> {
    type Impl = ResultImpl<E>;
}

impl<E> FunctorOnceImpl for ResultImpl<E> {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.map(f)
    }
}
