/// This links the higher-kinded type constructor to an instance of a type.
/// For example, this is implemented for `Option<T>` and provides the type
/// `OptionImpl`. This is co-constrained by the trait `HigherImpl` which must
/// also be implemented on the correct type.
pub trait Higher<T> {
    type Impl: HigherImpl<Kind<T> = Self>;
}

/// This allows a type to provide instances of another type that have a generic
/// parameter. For example, this is implemented for `OptionImpl` and has
/// `Kind<T> = Option<T>`. This is co-constrained by the trait `Higher` which
/// must also be implemented on the correct type.
pub trait HigherImpl {
    type Kind<T>: Higher<T, Impl = Self>;
}

/// The traits `Higher` and `HigherImpl` are co-constrained and this macro
/// provides an implementation for simple higher kinded types such as `Option`
/// or `Vec`. This doesn't work for more complex types such as `Result<T, E>`.
#[macro_export]
macro_rules! implHigher {
    ($TypeIdent:ident, $ImplIdent:ident) => {
        impl HigherImpl for $ImplIdent {
            type Kind<T> = $TypeIdent<T>;
        }
        impl<T> Higher<T> for $TypeIdent<T> {
            type Impl = $ImplIdent;
        }
    };
}

/// An ergonomic trait that is blanket implemented to avoid nested use of
/// fully qualified syntax.
pub trait HigherWith<A>: Higher<A> {
    type With<T>: Higher<T, Impl = Self::Impl>;
}

impl<A, T: Higher<A>> HigherWith<A> for T {
    type With<B> = <<Self as Higher<A>>::Impl as HigherImpl>::Kind<B>;
}
