/// This links the higher-kinded type constructor to an instance of a type.
/// For example, this is implemented for `Option<T>` and provides the type
/// `OptionImpl`. This is co-constrained by the trait `HigherImpl` which must
/// also be implemented on the correct type.
pub trait HigherKind {
    type Item;
    type Impl: HigherImpl<Kind<Self::Item> = Self>;
}

/// This allows a type to provide instances of another type that have a generic
/// parameter. For example, this is implemented for `OptionImpl` and has
/// `Kind<T> = Option<T>`. This is co-constrained by the trait `HigherKind` which
/// must also be implemented on the correct type.
pub trait HigherImpl {
    type Kind<T>: HigherKind<Item = T, Impl = Self>;
}

/// The traits `HigherImpl` and `HigherKind` are co-constrained and this macro
/// provides an implementation for simple higher kinded types such as `Option`
/// or `Vec`. This doesn't work for more complex types such as `Result<T, E>`.
#[macro_export]
macro_rules! implHigher {
    ($TypeIdent:ident, $ImplIdent:ident) => {
        impl HigherImpl for $ImplIdent {
            type Kind<T> = $TypeIdent<T>;
        }
        impl<T> HigherKind for $TypeIdent<T> {
            type Item = T;
            type Impl = $ImplIdent;
        }
    };
}

/// An ergonomic trait that is blanket implemented to avoid nested use of
/// fully qualified syntax.
pub trait Higher: HigherKind {
    type With<T>: HigherKind<Item = T, Impl = Self::Impl>;
}

impl<T: HigherKind> Higher for T {
    type With<B> = <<Self as HigherKind>::Impl as HigherImpl>::Kind<B>;
}
