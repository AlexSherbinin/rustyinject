//! Injector trait, containers and strategies needed for dependency injection.

mod constructor_factory;
mod factory;
mod singleton;
mod singleton_cloned;

pub mod strategies {
    //! Strategies of dependency injection.
    pub use super::constructor_factory::{
        ConstructorFactoryStrategy, RefConstructorFactoryStrategy,
    };
    pub use super::factory::{FactoryStrategy, RefFactoryStrategy};
    pub use super::singleton::SingletonStrategy;
    pub use super::singleton_cloned::SinglentonClonedStrategy;
}

pub mod containers {
    //! Containers for storing dependencies.
    pub use super::constructor_factory::{
        ConstructorFactoryContainer, RefConstructorFactoryContainer,
    };
    pub use super::factory::{FactoryContainer, RefFactoryContainer};
    pub use super::singleton::SingletonContainer;
}

pub mod factories {
    //! Factories used for creation instances of structs that depend on others.
    pub use super::constructor_factory::{ConstructorFactory, RefConstructorFactory};
    pub use super::factory::{Factory, RefFactory};
}

/// A trait for performing dependency injection.
/// It serves as a generic interface for implementing dependency injection logic.
pub trait Injector<T, Infer> {
    /// Inject a dependency.
    fn inject(self) -> T;
}

/// A trait for performing dependency injection of many dependencies at once.
pub trait ListInjector<T, Infer> {
    /// Inject a list of dependencies.
    fn inject_list(self) -> T;
}

impl<'a, Head, Tail, HeadInfer, TailInfer, C> ListInjector<(Head, Tail), (HeadInfer, TailInfer)>
    for &'a C
where
    &'a C: Injector<Head, HeadInfer> + ListInjector<Tail, TailInfer>,
{
    fn inject_list(self) -> (Head, Tail) {
        (self.inject(), self.inject_list())
    }
}

impl<C> ListInjector<(), ()> for &C {
    fn inject_list(self) {}
}

#[cfg(test)]
mod tests {
    use crate::DependencyContainer;

    use super::*;

    #[test]
    fn test_inject_list() {
        #[derive(Clone)]
        struct Database;
        struct Cache;

        let container = DependencyContainer::default()
            .with_singleton(Database)
            .with_singleton(Cache);

        let _deps: (Database, (&Cache, ())) = (&container).inject_list();
    }
}
