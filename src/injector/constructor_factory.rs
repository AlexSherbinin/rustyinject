use super::{Injector, ListInjector};
use crate::{DependencyContainer, DepsListGetRef};
use core::convert::Infallible;
use core::marker::PhantomData;

/// A trait representing a struct that builds from a constructor.
pub trait ConstructorFactory: Sized {
    /// Dependencies of the factory.
    type Dependencies<'a>;

    /// Creates a new instance from dependencies.
    fn build(dependencies: Self::Dependencies<'_>) -> Self;
}

/// A trait representing a struct that builds from a constructor but instead of
/// [`ConstructorFactory`] consumes all references that passed.
pub trait RefConstructorFactory<'a>: Sized {
    /// Dependencies of the factory.
    type Dependencies;

    /// Create a new instance of dependencies.
    fn build(dependencies: Self::Dependencies) -> Self;
}

/// A marker struct used to signify a factory strategy in dependency injection.
pub struct ConstructorFactoryStrategy<ConstructorInfer>(PhantomData<ConstructorInfer>, Infallible);
/// A marker struct used to signify the factory strategy with consuming references of dependencies
/// in dependency injection.
pub struct RefConstructorFactoryStrategy<ConstructorInfer>(
    PhantomData<ConstructorInfer>,
    Infallible,
);

/// A container for holding type of struct that can be built from specified dependencies.
pub struct ConstructorFactoryContainer<T>(pub(crate) PhantomData<T>);
/// A container for holding type of struct that can be built from specified dependencies by
/// consuming references to them.
pub struct RefConstructorFactoryContainer<T>(pub(crate) PhantomData<T>);

impl<'a, Parent, Scope, ConstructorInfer, T, Infer>
    Injector<T, (Infer, ConstructorFactoryStrategy<ConstructorInfer>)>
    for &'a DependencyContainer<Parent, Scope>
where
    Self: DepsListGetRef<ConstructorFactoryContainer<T>, Infer>
        + ListInjector<T::Dependencies<'a>, ConstructorInfer>,
    T: ConstructorFactory,
{
    fn inject(self) -> T {
        T::build(self.inject_list())
    }
}

impl<'a, Parent, Scope, ConstructorInfer, T, Infer>
    Injector<T, (Infer, RefConstructorFactoryStrategy<ConstructorInfer>)>
    for &'a DependencyContainer<Parent, Scope>
where
    Self: DepsListGetRef<RefConstructorFactoryContainer<T>, Infer>
        + ListInjector<T::Dependencies, ConstructorInfer>,
    T: RefConstructorFactory<'a>,
{
    fn inject(self) -> T {
        T::build(self.inject_list())
    }
}

#[cfg(test)]
mod tests {
    use crate::injector::{factories::RefConstructorFactory, Injector};
    use crate::{injector::constructor_factory::ConstructorFactory, DependencyContainer};

    #[test]
    fn test_inject() {
        #[derive(Clone)]
        struct Database;
        struct Cache;

        struct App(Database);

        impl ConstructorFactory for App {
            type Dependencies<'a> = (Database, ());

            fn build(dependencies: Self::Dependencies<'_>) -> Self {
                Self(dependencies.0)
            }
        }

        #[allow(dead_code)]
        struct AnotherApp<'a>(Database, &'a Cache);

        impl<'a> RefConstructorFactory<'a> for AnotherApp<'a> {
            type Dependencies = (Database, (&'a Cache, ()));

            fn build(dependencies: Self::Dependencies) -> Self {
                Self::new(dependencies.0, dependencies.1 .0)
            }
        }

        impl<'a> AnotherApp<'a> {
            fn new(db: Database, cache: &'a Cache) -> Self {
                Self(db, cache)
            }
        }

        let container = DependencyContainer::default()
            .with_singleton(Database)
            .with_singleton(Cache)
            .with_constructor_factory::<App>()
            .with_ref_constructor_factory::<AnotherApp>();

        let _app: App = (&container).inject();
        let _another_app: AnotherApp = (&container).inject();
    }
}
