use super::Injector;
use crate::{container::DependencyContainer, deps_list::DepsListGet};
use std::{convert::Infallible, marker::PhantomData};

pub struct FactoryStrategy<F, FactoryData>(PhantomData<(F, FactoryData)>, Infallible);

pub trait Factory {
    type Result;
}

pub trait FactoryBuild<DepsContainer, Data>: Factory {
    fn build(&self, container: &DepsContainer) -> Self::Result;
}

pub struct FactoryContainer<F, FactoryResult>(pub(crate) F, pub(crate) PhantomData<FactoryResult>);

impl<Parent, Scope, F, FactoryData, T, Idx> Injector<T, Idx, FactoryStrategy<F, FactoryData>>
    for &DependencyContainer<Parent, Scope>
where
    Scope: DepsListGet<FactoryContainer<F, T>, Idx>,
    F: FactoryBuild<DependencyContainer<Parent, Scope>, FactoryData, Result = T>,
{
    fn inject(self) -> T {
        let factory = &self.scope.get().0;
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

        struct App {
            db: Database,
            cache: Cache,
        }

        struct AppFactory;

        impl Factory for AppFactory {
            type Result = App;
        }

        impl<DepsContainer, DatabaseIdx, DatabaseStrategy, CacheIdx, CacheStrategy>
            FactoryBuild<DepsContainer, (DatabaseIdx, DatabaseStrategy, CacheIdx, CacheStrategy)>
            for AppFactory
        where
            for<'a> &'a DepsContainer: Injector<&'a Database, DatabaseIdx, DatabaseStrategy>
                + Injector<&'a Cache, CacheIdx, CacheStrategy>,
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

        let app: App = (&container).inject();
    }
}
