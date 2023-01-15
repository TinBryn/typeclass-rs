use crate::prelude::*;

pub struct OptionImpl;

implHigher!(Option, OptionImpl);

impl FunctorOnceImpl for OptionImpl {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.map(f)
    }
}

impl PointImpl for OptionImpl {
    fn point<A>(a: A) -> Self::Kind<A> {
        Some(a)
    }
}

impl ApplyOnceImpl for OptionImpl {
    fn apply_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        fa.and_then(|a| ff.map(|f| f(a)))
    }
}

impl BindOnceImpl for OptionImpl {
    fn bind_once<A, B, F: FnOnce(A) -> Self::Kind<B>>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.and_then(f)
    }
}
