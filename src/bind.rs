use crate::prelude::Higher;

pub trait Bind: Higher {
    fn bind<B, F: Fn(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindMut: Bind {
    fn bind_mut<B, F: FnMut(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

pub trait BindOnce: BindMut {
    fn bind_once<B, F: FnOnce(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

impl<T: BindOnce> BindMut for T {
    fn bind_mut<B, F: FnMut(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        self.bind_once(f)
    }
}

impl<T: BindMut> Bind for T {
    fn bind<B, F: Fn(Self::Item) -> Self::With<B>>(self, f: F) -> Self::With<B> {
        self.bind_mut(f)
    }
}
