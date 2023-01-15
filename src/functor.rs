mod implementation;
pub use implementation::{FunctorImpl, FunctorMutImpl, FunctorOnceImpl};

use crate::higher::{Higher, HigherWith};

pub trait Functor<A>: HigherWith<A>
where
    Self::Impl: FunctorImpl,
{
    //! Functor ergonomic trait, this gets blanket implemented when the
    //! `FunctorImpl` trait is implemented and is how this typeclass should be
    //! used.
    //!
    //! note: the constraint `Self::Impl: FunctorImpl` doesn't seem to remove the
    //! need to specify this constraint when used. It would be nice if it could be
    //! inferred based on this trait having it.
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::With<B>;
}

pub trait FunctorMut<A>: Functor<A>
where
    Self::Impl: FunctorMutImpl,
{
    //! `FunctorMut` ergonomic trait, this gets blanket implemented when the
    //! `FunctorMutImpl` trait is implemented and is how this typeclass should be
    //! used. This can be used as a `Functor` as it will just be provided with a
    //! `FnMut` that doesn't actually mutate anything.
    //!
    //! note: the constraint `Self::Impl: FunctorMutImpl` doesn't seem to remove the
    //! need to specify this constraint when used. It would be nice if it could be
    //! inferred based on this trait having it.
    fn fmap_mut<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B>;
}

pub trait FunctorOnce<A>: FunctorMut<A>
where
    Self::Impl: FunctorOnceImpl,
{
    //! `FunctorOnce` ergonomic trait, this gets blanket implemented when the
    //! `FunctorOnceImpl` trait is implemented and is how this typeclass should be
    //! used. This can be used as a `FunctorMut`.
    //!
    //! note: the constraint `Self::Impl: FunctorOnceImpl` doesn't seem to remove the
    //! need to specify this constraint when used. It would be nice if it could be
    //! inferred based on this trait having it.
    fn fmap_once<B, F: FnOnce(A) -> B>(self, f: F) -> Self::With<B>;
}

impl<A, T: Higher<A>> Functor<A> for T
where
    Self::Impl: FunctorImpl,
{
    // blanket implementation for `Functor` from `FunctorImpl`
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Self::With<B> {
        Self::Impl::fmap(self, f)
    }
}

impl<A, T: Higher<A>> FunctorMut<A> for T
where
    Self::Impl: FunctorMutImpl,
{
    // blanket implementation for `FunctorMut` from `FunctorMutImpl`
    fn fmap_mut<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B> {
        Self::Impl::fmap_mut(self, f)
    }
}

impl<A, T: Higher<A>> FunctorOnce<A> for T
where
    Self::Impl: FunctorOnceImpl,
{
    // blanket implementation for `FunctorOnce` from `FunctorOnceImpl`
    fn fmap_once<B, F: FnOnce(A) -> B>(self, f: F) -> Self::With<B> {
        Self::Impl::fmap_once(self, f)
    }
}
