use crate::{
    higher::HigherImpl,
    prelude::{Higher, HigherKind},
};

pub trait PointImpl: HigherImpl {
    fn point<A>(a: A) -> Self::Kind<A>;
}

pub trait Point: Higher {
    fn point<A>(b: A) -> Self::With<A>;
}

impl<T: HigherKind> Point for T
where
    Self::Impl: PointImpl,
{
    fn point<A>(b: A) -> Self::With<A> {
        Self::Impl::point(b)
    }
}
