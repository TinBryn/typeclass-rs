use crate::{
    higher::HigherImpl,
    prelude::{Higher, HigherKind},
};

pub trait PointImpl: HigherImpl {
    fn point<A>(a: A) -> Self::Kind<A>;
}

pub trait Point<A>: Higher<A> {
    fn point<B>(b: B) -> Self::With<B>;
}

impl<A, T: HigherKind<A>> Point<A> for T
where
    Self::Impl: PointImpl,
{
    fn point<B>(b: B) -> Self::With<B> {
        Self::Impl::point(b)
    }
}
