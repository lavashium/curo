pub trait Context {}

pub trait Driver {
    type Context: Context;
}

pub trait Factory<Product, Source>: Driver {
    fn run(source: &mut Source, ctx: &mut Self::Context) -> Product;

    fn run_box(boxed: &mut Box<Source>, ctx: &mut Self::Context) -> Product {
        Self::run(boxed.as_mut(), ctx)
    }

    fn run_option(option: &mut Option<Source>, ctx: &mut Self::Context) -> Option<Product> {
        option.as_mut().map(|source| Self::run(source, ctx))
    }

    fn run_option_box(option: &mut Option<Box<Source>>, ctx: &mut Self::Context) -> Option<Product> {
        option.as_mut().map(|boxed| Self::run(&mut **boxed, ctx))
    }
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



#[macro_export]
macro_rules! factory_list {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        Then<$head, factory_list!($($tail),*)>
    };
}

pub struct Then<A, B> (pub A, pub B);

pub trait FactoryList<Product, Source, Context> 
where
    Context: self::Context, {

    fn run(source: &mut Source, ctx: &mut Context) -> Product;
}

impl<Product, Source, Context> FactoryList<Product, Source, Context> for () 
where
    Product: Default,
    Context: self::Context, {

    #[inline(always)]
    fn run(_source: &mut Source, _ctx: &mut Context) -> Product {
        Product::default()
    }
}

impl<Product, Source, Context, Head, Tail> FactoryList<Product, Source, Context> for Then<Head, Tail>
where
    Head: Factory<Product, Source> + Driver<Context = Context>,
    Tail: FactoryList<Product, Source, Context>,
    Product: Chain<Product>,
    Context: self::Context, {

    #[inline(always)]
    fn run(source: &mut Source, ctx: &mut Context) -> Product {
        let head_result = Head::run(source, ctx);
        Product::chain(head_result, || { Tail::run(source, ctx) })
    }
}