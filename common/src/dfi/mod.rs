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
    #[inline(always)]
    fn chain(lhs: Option<T>, rhs: impl FnOnce() -> Option<T>) -> Option<T> {
        lhs.or_else(rhs)
    }
}

impl Chain<()> for () {
    #[inline(always)]
    fn chain(_: (), f: impl FnOnce() -> ()) -> () {
        f()
    }
}

pub trait Factory<T, Source, Context> {
    fn run(source: &mut Source, ctx: &mut Context) -> T;

    fn run_box(boxed: &mut Box<Source>, ctx: &mut Context) -> T {
        Self::run(boxed.as_mut(), ctx)
    }

    fn run_option(option: &mut Option<Source>, ctx: &mut Context) -> Option<T> {
        option.as_mut().map(|source| Self::run(source, ctx))
    }

    fn run_option_box(option: &mut Option<Box<Source>>, ctx: &mut Context) -> Option<T> {
        option.as_mut().map(|boxed| Self::run(&mut **boxed, ctx))
    }
}

pub trait FactoryList<T, Source, Context> {
    fn run(source: &mut Source, ctx: &mut Context) -> T;
}

impl<T: Default, Source, Context> FactoryList<T, Source, Context> for () {
    fn run(_source: &mut Source, _ctx: &mut Context) -> T {
        T::default()
    }
}

impl<T, Source, Context, Head, Tail> FactoryList<T, Source, Context> for (Head, Tail)
where
    Head: Factory<T, Source, Context>,
    Tail: FactoryList<T, Source, Context>,
    T: Chain<T> {

    fn run(source: &mut Source, ctx: &mut Context) -> T {
        let head_result = Head::run(source, ctx);
        T::chain(head_result, || Tail::run(source, ctx))
    }
}