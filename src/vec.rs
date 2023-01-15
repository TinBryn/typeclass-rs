use crate::prelude::*;

pub struct VecImpl;

implHigher!(Vec, VecImpl);

impl FunctorMutImpl for VecImpl {
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.into_iter().map(f).collect()
    }
}

impl ApplyMutImpl for VecImpl {
    fn apply_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        ff.into_iter().zip(fa).map(|(mut f, a)| f(a)).collect()
    }
}

impl BindMutImpl for VecImpl {
    fn bind_mut<A, B, F: FnMut(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.into_iter().flat_map(f).collect()
    }
}
