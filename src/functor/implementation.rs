use crate::higher::HigherImpl;

pub trait FunctorImpl: HigherImpl {
    //! This trait is for implementing a functor on a higher kinded impl.
    //! This trait is blanket implemented for everything that implements
    //! `FunctorMutImpl` and if that is possible to implement, is prefered.

    /// Uses the provided callable to convert a `Kind<A>` into a `Kind<B>`
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait FunctorMutImpl: FunctorImpl {
    //! This trait is for implementing a variation of a functor that allows a
    //! `FnMut` as the mapping function. This trait is blanket implemented for
    //! everything that implements `FunctorOnceImpl` and if that is possible to
    //! implement, is prefered. Also if this trait is implemented, there is a
    //! blanket implementation for `FunctorImpl`.
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait FunctorOnceImpl: FunctorMutImpl {
    //! This trait is for implementing a variation of a functor that allows a
    //! `FnOnce` as the mapping function. If this trait is implemented, there is a
    //! blanket implementation for `FunctorMutImpl`, which will then trigger a
    //! blanket implementation for `FunctorImpl`. This is the most capable version
    //! and if possible to implement is prefered. For an example of these
    //! capabilities, `OptionImpl` implements `FunctorOnceImpl`, but `VecImpl` only
    //! implements `FunctorMutImpl`
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

impl<T: FunctorMutImpl> FunctorImpl for T {
    // Functor blanket impl if FunctorMut is implemented
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::fmap_mut(fa, f)
    }
}

impl<T: FunctorOnceImpl> FunctorMutImpl for T {
    // blanket implementation for `FunctorMutImpl` from `FunctorOnceImpl`.
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::fmap_once(fa, f)
    }
}
