#![feature(never_type)]

pub mod apply;
pub mod bind;
pub mod functor;
pub mod higher;
pub mod point;

pub mod prelude {
    pub use crate::apply::*;
    pub use crate::bind::*;
    pub use crate::functor::*;
    pub use crate::higher::*;
    pub use crate::implHigher;
    pub use crate::point::*;
}

mod option;
mod vec;
mod result;

#[cfg(test)]
mod test {
    use crate::prelude::*;

    fn ints_to_strs<F>(ints: F) -> F::With<String>
    where
        F: Functor<i32>,
        F::Impl: FunctorImpl,
    {
        Functor::fmap(ints, |i| i.to_string())
    }

    #[test]
    fn option_fmap_ints_to_strs() {
        let ints = Some(3);
        let strs = ints_to_strs(ints);
        assert_eq!(strs, Some("3".to_string()));
    }

    #[test]
    fn vec_fmap_ints_to_strs() {
        let ints = vec![1, 2, 3];
        let strs = ints_to_strs(ints);
        assert_eq!(
            strs,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn result_fmap_ints_to_strs() {
        let ints: Result<_, !> = Ok(3);
        let strs = ints_to_strs(ints);
        assert_eq!(strs, Ok("3".to_string()));
    }
}
