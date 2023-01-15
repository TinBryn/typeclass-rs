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
mod result;
mod vec;

#[cfg(test)]
mod test {
    use crate::prelude::*;

    fn to_strs<F: Functor>(ints: F) -> F::With<String>
    where
        F::Item: ToString,
    {
        Functor::fmap(ints, |i| i.to_string())
    }

    fn to_strs_with_index<F: FunctorMut>(ints: F) -> F::With<(usize, String)>
    where
        F::Item: ToString,
    {
        let mut index = 0;
        FunctorMut::fmap_mut(ints, |i| {
            let result = (index, i.to_string());
            index += 1;
            result
        })
    }

    #[test]
    fn option_fmap_ints_to_strs() {
        let ints = Some(3);
        let strs = to_strs(ints);
        assert_eq!(strs, Some("3".to_string()));
    }

    #[test]
    fn vec_fmap_ints_to_strs() {
        let ints = vec![1, 2, 3];
        let strs = to_strs(ints);
        assert_eq!(
            strs,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn result_fmap_ints_to_strs() {
        let ints: Result<_, !> = Ok(3);
        let strs = to_strs(ints);
        assert_eq!(strs, Ok("3".to_string()));
    }

    #[test]
    fn fmap_with_mutable_functor_on_vec() {
        let ints = vec![1, 2, 3];
        let strs = to_strs_with_index(ints);
        assert_eq!(
            strs,
            vec![
                (0, "1".to_string()),
                (1, "2".to_string()),
                (2, "3".to_string())
            ]
        );
    }
}
