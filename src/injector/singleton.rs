use super::Injector;
use crate::{
    container::DependencyContainer,
    deps_list::{DepsListGetMut, DepsListGetRef, DepsListRemove},
};
use core::convert::Infallible;

/// A marker struct used to signify the singleton strategy in dependency injection.
pub struct SingletonStrategy(Infallible);
/// A container for holding a singleton instance of a dependency.
pub struct SingletonContainer<T>(pub(crate) T);

impl<Parent, Scope, T, Infer>
    Injector<
        (
            T,
            <Self as DepsListRemove<SingletonContainer<T>, Infer>>::Remainder,
        ),
        (Infer, SingletonStrategy),
    > for DependencyContainer<Parent, Scope>
where
    Self: DepsListRemove<SingletonContainer<T>, Infer>,
{
    /// Split the container into a dependency(singleton) and a new container with the dependency removed.
    fn inject(
        self,
    ) -> (
        T,
        <Self as DepsListRemove<SingletonContainer<T>, Infer>>::Remainder,
    ) {
        let (injected, new_container) = self.remove();

        (injected.0, new_container)
    }
}

impl<'a, Parent, Scope, T, Infer> Injector<&'a T, (Infer, SingletonStrategy)>
    for &'a DependencyContainer<Parent, Scope>
where
    DependencyContainer<Parent, Scope>: DepsListGetRef<SingletonContainer<T>, Infer>,
{
    /// Inject an immutable reference to the dependency(singleton) by consuming an immutable reference to the container.
    fn inject(self) -> &'a T {
        &self.get().0
    }
}

impl<'a, Parent, Scope, T, Infer> Injector<&'a mut T, (Infer, SingletonStrategy)>
    for &'a mut DependencyContainer<Parent, Scope>
where
    DependencyContainer<Parent, Scope>: DepsListGetMut<SingletonContainer<T>, Infer>,
{
    /// Inject a mutable reference to the dependency(singleton) by consuming a mutable reference to the container.
    fn inject(self) -> &'a mut T {
        &mut self.get_mut().0
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
