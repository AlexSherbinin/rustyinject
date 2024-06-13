use crate::{
    deps_list::{DepsList, DepsListGetMut, DepsListGetRef, DepsListRemove},
    injector::{
        containers::{
            ConstructorFactoryContainer, FactoryContainer, RefConstructorFactoryContainer,
            RefFactoryContainer, SingletonContainer,
        },
        factories::{ConstructorFactory, Factory, RefFactory},
    },
};
use core::{convert::Infallible, marker::PhantomData};

/// Current scope index.
pub struct CurrentScope(Infallible);
/// Parent scope index.
pub struct ParentScope<Scope>(PhantomData<Scope>, Infallible);

/// An Inversion of Control (IoC) container used for declaring and managing dependencies in a Rust application.
/// It facilitates the creation, storage, and retrieval of dependencies, supporting both singleton and factory-based dependency injection.
/// # Generics
/// - `Parent`: a parent container.
/// - `Scope`: current scope of the container.
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
    /// Create a new container with the specified parent.
    pub fn new(parent: Parent) -> Self {
        Self { parent, scope: () }
    }
}

impl<Parent, Scope> DependencyContainer<Parent, Scope>
where
    Scope: DepsList,
{
    /// Add a concrete instance of a dependency (singleton) to the container.
    pub fn with_singleton<T>(
        self,
        singleton: T,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<SingletonContainer<T>>> {
        DependencyContainer {
            parent: self.parent,
            scope: self.scope.prepend(SingletonContainer(singleton)),
        }
    }

    /// Add a factory-based dependency to the container.
    pub fn with_factory<F>(
        self,
        factory: F,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<FactoryContainer<F, F::Result>>>
    where
        F: Factory,
    {
        DependencyContainer {
            parent: self.parent,
            scope: self.scope.prepend(FactoryContainer(factory, PhantomData)),
        }
    }

    /// Add a factory-based dependency to the container.
    pub fn with_ref_factory<'a, F>(
        self,
        factory: F,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<RefFactoryContainer<F, F::Result<'a>>>>
    where
        F: RefFactory,
    {
        DependencyContainer {
            parent: self.parent,
            scope: self
                .scope
                .prepend(RefFactoryContainer(factory, PhantomData)),
        }
    }

    /// Add a struct that builds from a constructor(like a `new` method).
    pub fn with_constructor_factory<T>(
        self,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<ConstructorFactoryContainer<T>>>
    where
        T: ConstructorFactory,
    {
        DependencyContainer {
            parent: self.parent,
            scope: self.scope.prepend(ConstructorFactoryContainer(PhantomData)),
        }
    }

    /// Add a struct that builds from a constructor(like a `new` method) and consumes all
    /// references that passed
    pub fn with_ref_constructor_factory<T>(
        self,
    ) -> DependencyContainer<Parent, Scope::PrependedWith<RefConstructorFactoryContainer<T>>> {
        DependencyContainer {
            parent: self.parent,
            scope: self
                .scope
                .prepend(RefConstructorFactoryContainer(PhantomData)),
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
