//! 首先，这个文件定义了一些宏，这些宏用于将新类型迭代器（newtype iterators）的实现委托给内部类型。新类型迭代器是一种常见的设计模式，它允许你创建一个新的类型，这个新的类型在运行时与其内部类型完全相同，但在编译时被视为一个不同的类型。这种设计模式可以帮助你提高类型安全性，防止类型混淆。

/// 这些宏在实现时将 impl 约束放在了最后，因为这是作者知道的唯一一种可以消耗任意列表约束的方法，这种方法使用了 $($args:tt)* 这样的模式来匹配任意数量的 token。

/// 其中一个宏的功能是创建一个并行迭代器（parallel iterator）的实现，这个实现简单地包装了一个内部类型，并将所有方法委托给这个内部类型。要使用这个宏，你需要先声明一个具有 inner 字段的结构体。

/// 需要注意的是，IntoParallelIterator 的实现需要单独添加，这个宏不会为你添加这个实现。

// 这个宏接受两个参数：一个是迭代器类型 $iter，另一个是迭代器元素类型 $item。它还接受一个 impl 块，其中可以包含任意数量的 trait 约束。
macro_rules! delegate_iterator {
  ($iter:ty => $item:ty,
   impl $( $args:tt)*
  ) => {
    impl $( $args)* ParallelIterator for $iter {
      type Item = $item;

      fn drive_unindexed<C>(self, consumer: C) -> C::Result
      where
        C: UnindexedConsumer<Self::Item>,
      {
        self.inner.drive_unindexed(consumer)
      }

      fn opt_len(&self) -> Option<usize> {
        self.inner.opt_len()
      }
    }
  }
}

macro_rules! delegate_indexed_iterator {
  ($iter:ty => $item:ty ,
   impl $( $args:tt )*
   ) => {
      delegate_iterator!{
          $iter => $item ,
          impl $( $args )*
      }

      impl $( $args )* IndexedParallelIterator for $iter {
          fn drive<C>(self, consumer: C) -> C::Result
              where C: Consumer<Self::Item>
          {
              self.inner.drive(consumer)
          }

          fn len(&self) -> usize {
              self.inner.len()
          }

          fn with_producer<CB>(self, callback: CB) -> CB::Output
              where CB: ProducerCallback<Self::Item>
          {
              self.inner.with_producer(callback)
          }
      }
  }
}
