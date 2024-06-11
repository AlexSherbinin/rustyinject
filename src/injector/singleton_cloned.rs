use super::{containers::SingletonContainer, Injector};
use crate::{container::DependencyContainer, deps_list::DepsListGetRef};
use std::convert::Infallible;

/// A marker struct used to signify the singleton cloned strategy in dependency injection.
pub struct SinglentonClonedStrategy(Infallible);

impl<Parent, Scope, T, Infer> Injector<T, (Infer, SinglentonClonedStrategy)>
    for &DependencyContainer<Parent, Scope>
where
    DependencyContainer<Parent, Scope>: DepsListGetRef<SingletonContainer<T>, Infer>,
    T: Clone,
{
    /// Inject a dependency(singleton) by cloning it from the container
    fn inject(self) -> T {
        self.get().0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject() {
        #[derive(Clone)]
        struct Database;

        let container = DependencyContainer::default().with_singleton(Database);

        let _db: &Database = (&container).inject();
        let _db: Database = (&container).inject();
    }
}
