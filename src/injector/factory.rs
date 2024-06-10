use super::Injector;
use crate::{container::DependencyContainer, deps_list::DepsListGetRef};
use std::{convert::Infallible, marker::PhantomData};

pub struct FactoryStrategy<F, FactoryData>(PhantomData<(F, FactoryData)>, Infallible);

pub trait Factory {
    type Result;
}

pub trait FactoryBuild<DepsContainer, Data>: Factory {
    fn build(&self, container: &DepsContainer) -> Self::Result;
}

pub struct FactoryContainer<F, FactoryResult>(pub(crate) F, pub(crate) PhantomData<FactoryResult>);

impl<Parent, Scope, F, FactoryData, T, Infer> Injector<T, (Infer, FactoryStrategy<F, FactoryData>)>
    for &DependencyContainer<Parent, Scope>
where
    Self: DepsListGetRef<FactoryContainer<F, T>, Infer>,
    F: FactoryBuild<DependencyContainer<Parent, Scope>, FactoryData, Result = T>,
{
    fn inject(self) -> T {
        let factory = &self.get().0;
        factory.build(self)
    }
}

#[cfg(test)]
mod tests {
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
        }

        impl<DepsContainer, DatabaseInfer> FactoryBuild<DepsContainer, DatabaseInfer> for AnotherAppFactory
        where
            for<'a> &'a DepsContainer: Injector<&'a Database, DatabaseInfer>,
        {
            fn build(&self, container: &DepsContainer) -> Self::Result {
                AnotherApp(container.inject().clone())
            }
        }

        struct App {
            db: Database,
            cache: Cache,
        }

        struct AppFactory;

        impl Factory for AppFactory {
            type Result = App;
        }

        impl<DepsContainer, DatabaseInfer, CacheInfer>
            FactoryBuild<DepsContainer, (DatabaseInfer, CacheInfer)> for AppFactory
        where
            for<'a> &'a DepsContainer:
                Injector<&'a Database, DatabaseInfer> + Injector<&'a Cache, CacheInfer>,
        {
            fn build(&self, container: &DepsContainer) -> Self::Result {
                let db: &Database = container.inject();
                let cache: &Cache = container.inject();

                App {
                    db: db.clone(),
                    cache: cache.clone(),
                }
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
