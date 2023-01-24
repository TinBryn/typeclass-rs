pub trait Higher {
    type Item;
    type With<T>: Higher<With<Self::Item> = Self>;
}
