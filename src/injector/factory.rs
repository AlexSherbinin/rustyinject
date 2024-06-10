use super::Injector;
use crate::{
    container::DependencyContainer,
    deps_list::{DepsListGet, PredicateMatches},
};
use std::{convert::Infallible, marker::PhantomData};

pub struct FactoryStrategy<F, FactoryData>(PhantomData<(F, FactoryData)>, Infallible);

pub trait Factory<DepsContainer, Data> {
    type Result;

    fn build(&self, container: &DepsContainer) -> Self::Result;
}

pub struct FactoryContainer<F>(pub(crate) F);
pub struct FactoryPredicate<F, DepsContainer, FactoryData, FactoryResult>(
    PhantomData<(F, DepsContainer, FactoryData, FactoryResult)>,
    Infallible,
);
impl<F, DepsContainer, FactoryData, FactoryResult> PredicateMatches<FactoryContainer<F>>
    for FactoryPredicate<F, DepsContainer, FactoryData, FactoryResult>
where
    F: Factory<DepsContainer, FactoryData, Result = FactoryResult>,
{
}

impl<Parent, Scope, F, FactoryData, T, Idx> Injector<T, Idx, FactoryStrategy<F, FactoryData>>
    for &DependencyContainer<Parent, Scope>
where
    Scope: DepsListGet<
        FactoryPredicate<F, DependencyContainer<Parent, Scope>, FactoryData, T>,
        Idx,
        Value = FactoryContainer<F>,
    >,
    F: Factory<DependencyContainer<Parent, Scope>, FactoryData, Result = T>,
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

        impl<DepsContainer, DatabaseIdx, DatabaseStrategy, CacheIdx, CacheStrategy>
            Factory<DepsContainer, (DatabaseIdx, DatabaseStrategy, CacheIdx, CacheStrategy)>
            for AppFactory
        where
            for<'a> &'a DepsContainer: Injector<&'a Database, DatabaseIdx, DatabaseStrategy>
                + Injector<&'a Cache, CacheIdx, CacheStrategy>,
        {
            type Result = App;

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
