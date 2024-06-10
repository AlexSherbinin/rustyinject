use super::{Injector, SingletonContainer};
use crate::{container::DependencyContainer, deps_list::DepsListGetRef};
use std::convert::Infallible;

pub struct SinglentonClonedStrategy(Infallible);

impl<Parent, Scope, T, Infer> Injector<T, (Infer, SinglentonClonedStrategy)>
    for &DependencyContainer<Parent, Scope>
where
    DependencyContainer<Parent, Scope>: DepsListGetRef<SingletonContainer<T>, Infer>,
    T: Clone,
{
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
