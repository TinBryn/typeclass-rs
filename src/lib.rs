pub trait Higher {
    type Kind<T>: TypeConstructor<T, Instance = Self>;
}

pub trait TypeConstructor<T> {
    type Instance: Higher<Kind<T> = Self>;
}

pub trait FunctorOnce: Higher {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait FunctorMut: Higher {
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

pub trait Functor: Higher {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B>;
}

impl<T: FunctorOnce> FunctorMut for T {
    fn fmap_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::fmap_once(fa, f)
    }
}

impl<T: FunctorMut> Functor for T {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        Self::fmap_mut(fa, f)
    }
}

pub trait Point: Higher {
    fn point<A>(a: A) -> Self::Kind<A>;
}

pub trait ApplyOnce: Higher {
    fn apply_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

pub trait ApplyMut: Higher {
    fn apply_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

pub trait Apply: Higher {
    fn apply<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B>;
}

impl<T: ApplyOnce> ApplyMut for T {
    fn apply_mut<A, B, F: FnMut(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        Self::apply_once(fa, ff)
    }
}

impl<T: ApplyMut> Apply for T {
    fn apply<A, B, F: Fn(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        Self::apply_mut(fa, ff)
    }
}



pub struct OptionInstance;

impl Higher for OptionInstance {
    type Kind<T> = Option<T>;
}

impl<T> TypeConstructor<T> for Option<T> {
    type Instance = OptionInstance;
}

impl FunctorOnce for OptionInstance {
    fn fmap_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, f: F) -> Self::Kind<B> {
        fa.map(f)
    }
}

impl Point for OptionInstance {
    fn point<A>(a: A) -> Self::Kind<A> {
        Some(a)
    }
}

impl ApplyOnce for OptionInstance {
    fn apply_once<A, B, F: FnOnce(A) -> B>(fa: Self::Kind<A>, ff: Self::Kind<F>) -> Self::Kind<B> {
        fa.and_then(|a| ff.map(|f| f(a)))
    }
}
