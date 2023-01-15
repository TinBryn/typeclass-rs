use crate::higher::HigherImpl;

pub trait Point: HigherImpl {
    fn point<A>(a: A) -> Self::Kind<A>;
}
