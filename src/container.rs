use std::marker::PhantomData;

use crate::{
    deps_list::{DepsList, DepsListGet},
    injector::{Factory, FactoryBuild, FactoryContainer, SingletonContainer},
};

pub struct DependencyContainer<Parent, Scope> {
    pub(crate) parent: Parent,
    pub(crate) scope: Scope,
}

impl Default for DependencyContainer<(), ()> {
    fn default() -> Self {
        Self::new(())
    }
}

impl<Parent> DependencyContainer<Parent, ()> {
    pub fn new(parent: Parent) -> Self {
        Self { parent, scope: () }
    }
}

impl<Parent, Scope> DependencyContainer<Parent, Scope>
where
    Scope: DepsList,
{
    pub fn with_singleton<T>(
        self,
        singleton: T,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<SingletonContainer<T>>> {
        DependencyContainer {
            parent: self.parent,
            scope: self.scope.prepend(SingletonContainer(singleton)),
        }
    }

    pub fn with_factory<F, FactoryResult>(
        self,
        factory: F,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<FactoryContainer<F, FactoryResult>>>
    where
        F: Factory<Result = FactoryResult>,
    {
        DependencyContainer {
            parent: self.parent,
            scope: self.scope.prepend(FactoryContainer(factory, PhantomData)),
        }
    }
}
