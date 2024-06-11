mod container;
mod deps_list;
pub mod injector;

pub mod indecies {
    //! Indecies for indexing [`DepsList`](super::deps_list::DepsList) and
    //! [`DependencyContainer`](super::container::DependencyContainer)
    pub use super::container::{CurrentScope, ParentScope};
    pub use super::deps_list::{Last, Next};
}
pub use container::DependencyContainer;
pub use deps_list::{DepsList, DepsListGetMut, DepsListGetRef, DepsListRemove};
