//! Injector trait, containers and strategies needed for dependency injection.

mod factory;
mod singleton;
mod singleton_cloned;

/// Strategies of dependency injection.
pub mod strategies {
    pub use super::factory::FactoryStrategy;
    pub use super::singleton::SingletonStrategy;
    pub use super::singleton_cloned::SinglentonClonedStrategy;
}

/// Containers for storing dependencies.
pub mod containers {
    pub use super::factory::FactoryContainer;
    pub use super::singleton::SingletonContainer;
}

pub use factory::{Factory, FactoryBuild};

/// A trait for performing dependency injection.
/// It serves as a generic interface for implementing dependency injection logic.
pub trait Injector<T, Infer> {
    /// Inject an dependency.
    fn inject(self) -> T;
}
