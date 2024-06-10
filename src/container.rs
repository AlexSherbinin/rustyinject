use crate::{
    deps_list::{DepsList, DepsListGetMut, DepsListGetRef, DepsListRemove},
    injector::{CurrentScope, Factory, FactoryContainer, ParentScope, SingletonContainer},
};
use std::marker::PhantomData;

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

impl<Parent, Scope, T, Idx> DepsListRemove<T, (CurrentScope, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Scope: DepsListRemove<T, Idx>,
{
    type Remainder = DependencyContainer<Parent, Scope::Remainder>;

    fn remove(self) -> (T, Self::Remainder) {
        let (removed, scope_remainder) = self.scope.remove();
        (
            removed,
            DependencyContainer {
                parent: self.parent,
                scope: scope_remainder,
            },
        )
    }
}

impl<Parent, Scope, T, Idx, Subscope> DepsListRemove<T, (ParentScope<Subscope>, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Parent: DepsListRemove<T, (Subscope, Idx)>,
{
    type Remainder = DependencyContainer<Parent::Remainder, Scope>;

    fn remove(self) -> (T, Self::Remainder) {
        let (removed, parent_remainder) = self.parent.remove();

        (
            removed,
            DependencyContainer {
                parent: parent_remainder,
                scope: self.scope,
            },
        )
    }
}

impl<Parent, Scope, T, Idx> DepsListGetRef<T, (CurrentScope, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Scope: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        self.scope.get()
    }
}

impl<Parent, Scope, T, Idx, Subscope> DepsListGetRef<T, (ParentScope<Subscope>, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Parent: DepsListGetRef<T, (Subscope, Idx)>,
{
    fn get(&self) -> &T {
        self.parent.get()
    }
}

impl<Parent, Scope, T, Idx> DepsListGetMut<T, (CurrentScope, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Scope: DepsListGetMut<T, Idx>,
{
    fn get_mut(&mut self) -> &mut T {
        self.scope.get_mut()
    }
}

impl<Parent, Scope, T, Idx, Subscope> DepsListGetMut<T, (ParentScope<Subscope>, Idx)>
    for DependencyContainer<Parent, Scope>
where
    Parent: DepsListGetMut<T, (Subscope, Idx)>,
{
    fn get_mut(&mut self) -> &mut T {
        self.parent.get_mut()
    }
}
