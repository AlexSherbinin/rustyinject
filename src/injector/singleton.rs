use super::Injector;
use crate::{
    container::DependencyContainer,
    deps_list::{DepsListGet, DepsListRemove, PredicateMatches},
};
use std::{convert::Infallible, marker::PhantomData};

pub struct SingletonStrategy(Infallible);

pub struct SingletonContainer<T>(pub(crate) T);
pub struct SingletonPredicate<T>(PhantomData<T>, Infallible);
impl<T> PredicateMatches<SingletonContainer<T>> for SingletonPredicate<T> {}

impl<Parent, Scope, T, Idx>
    Injector<(T, DependencyContainer<Parent, Scope::Remainder>), Idx, SingletonStrategy>
    for DependencyContainer<Parent, Scope>
where
    Scope: DepsListRemove<SingletonPredicate<T>, Idx, Removed = SingletonContainer<T>>,
{
    fn inject(self) -> (T, DependencyContainer<Parent, Scope::Remainder>) {
        let (injected, new_scope) = self.scope.remove();
        (
            injected.0,
            DependencyContainer {
                parent: self.parent,
                scope: new_scope,
            },
        )
    }
}

impl<'a, Parent, Scope, T, Idx> Injector<&'a T, Idx, SingletonStrategy>
    for &'a DependencyContainer<Parent, Scope>
where
    Scope: DepsListGet<SingletonPredicate<T>, Idx, Value = SingletonContainer<T>>,
{
    fn inject(self) -> &'a T {
        &self.scope.get().0
    }
}

impl<'a, Parent, Scope, T, Idx> Injector<&'a mut T, Idx, SingletonStrategy>
    for &'a mut DependencyContainer<Parent, Scope>
where
    Scope: DepsListGet<SingletonPredicate<T>, Idx, Value = SingletonContainer<T>>,
{
    fn inject(self) -> &'a mut T {
        &mut self.scope.get_mut().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject() {
        struct App;
        struct AnotherApp;

        let mut container = DependencyContainer::default()
            .with_singleton(App)
            .with_singleton(AnotherApp);
        let _app: &App = (&container).inject();
        let _app: &mut App = (&mut container).inject();
        let (_app, _container): (App, _) = container.inject();
    }
}
