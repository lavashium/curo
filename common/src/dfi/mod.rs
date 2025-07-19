#[macro_export]
macro_rules! factory_pipeline {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        ($head, factory_pipeline!($($tail),*))
    };
}

pub trait Chain<T> {
    fn chain(lhs: T, rhs: impl FnOnce() -> T) -> T;
}

impl<T> Chain<Option<T>> for Option<T> {
    fn chain(lhs: Option<T>, rhs: impl FnOnce() -> Option<T>) -> Option<T> {
        lhs.or_else(rhs)
    }
}

impl Chain<()> for () {
    fn chain(_: (), _: impl FnOnce() -> ()) -> () {
        ()
    }
}

pub trait Factory<T, Driver, Context> {
    fn run(driver: &mut Driver, ctx: &mut Context) -> T;
}

pub trait FactoryList<T, Driver, Context> {
    fn run(driver: &mut Driver, ctx: &mut Context) -> T;
}

impl<T: Default, Driver, Context> FactoryList<T, Driver, Context> for () {
    fn run(_driver: &mut Driver, _ctx: &mut Context) -> T {
        T::default()
    }
}

impl<T, Driver, Context, Head, Tail> FactoryList<T, Driver, Context> for (Head, Tail)
where
    Head: Factory<T, Driver, Context>,
    Tail: FactoryList<T, Driver, Context>,
    T: Chain<T> {

    fn run(driver: &mut Driver, ctx: &mut Context) -> T {
        let head_result = Head::run(driver, ctx);
        T::chain(head_result, || Tail::run(driver, ctx))
    }
}
