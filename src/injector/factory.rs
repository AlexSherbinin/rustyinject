use super::{Injector, ListInjector};
use crate::{container::DependencyContainer, deps_list::DepsListGetRef};
use core::{convert::Infallible, marker::PhantomData};

/// A trait representing a factory for creating an instance of dependencies.
pub trait Factory {
    /// A result of the factory [`build`](Factory::build) method.
    type Result;
    /// Dependencies of the factory.
    type Dependencies;

    /// Build result from dependencies.
    fn build(&self, dependencies: Self::Dependencies) -> Self::Result;
}

/// A marker struct used to signify the factory strategy in dependency injection.
pub struct FactoryStrategy<F, FactoryInfer>(PhantomData<(F, FactoryInfer)>, Infallible);
/// A container for holding a factory instance and its result type.
pub struct FactoryContainer<F, FactoryResult>(pub(crate) F, pub(crate) PhantomData<FactoryResult>);

impl<Parent, Scope, F, FactoryInfer, T, Infer>
    Injector<T, (Infer, FactoryStrategy<F, FactoryInfer>)> for &DependencyContainer<Parent, Scope>
where
    Self:
        DepsListGetRef<FactoryContainer<F, T>, Infer> + ListInjector<F::Dependencies, FactoryInfer>,
    F: Factory<Result = T>,
{
    fn inject(self) -> T {
        let factory = &self.get().0;
        factory.build(self.inject_list())
    }
}

#[cfg(test)]
mod tests {
    use crate::DepsListRemove;

    use super::*;

    #[test]
    fn test_inject() {
        #[derive(Clone)]
        struct Database;
        #[derive(Clone)]
        struct Cache;

        struct AnotherApp(Database);

        struct AnotherAppFactory;

        impl Factory for AnotherAppFactory {
            type Result = AnotherApp;
            type Dependencies = (Database, ());

            fn build(&self, dependencies: Self::Dependencies) -> Self::Result {
                AnotherApp(dependencies.0)
            }
        }

        #[allow(dead_code)]
        struct App {
            db: Database,
            cache: Cache,
        }

        struct AppFactory;

        impl Factory for AppFactory {
            type Result = App;
            type Dependencies = (Database, (Cache, ()));

            fn build(&self, dependencies: Self::Dependencies) -> Self::Result {
                let (db, dependencies): (Database, (Cache, ())) = dependencies.remove();
                let (cache, _dependencies) = dependencies.remove();

                App { db, cache }
            }
        }

        let container = DependencyContainer::default()
            .with_singleton(Database)
            .with_singleton(Cache)
            .with_factory(AppFactory);
        let new_container = DependencyContainer::new(container).with_factory(AnotherAppFactory);

        let _app: App = (&new_container).inject();
        let _another_app: AnotherApp = (&new_container).inject();
    }
}
