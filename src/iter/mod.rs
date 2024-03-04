pub trait ParallelIterator: Sized + Send {
    type Item: Send;

    fn for_each<OP>(self, op: OP)
    where
        OP: Fn(Self::Item) + Sync + Send,
    {
        for_each::for_each(self, &op)
    }
}

pub trait IntoParallelIterator {
    type Iter: ParallelIterator<Item = Self::Item>;

    // Send trait 用于表示一个类型的值可以安全地在多个线程之间传递。如果一个类型实现了 Send，那么它的值可以被移动到另一个线程并在那里被安全地访问。
    type Item: Send;

    fn into_par_iter(self) -> Self::Iter;
}
