mod factory;
mod singleton;
mod singleton_cloned;

use std::{convert::Infallible, marker::PhantomData};

pub use factory::*;
pub use singleton::*;
pub use singleton_cloned::*;

pub struct CurrentScope(Infallible);
pub struct ParentScope<Scope>(PhantomData<Scope>, Infallible);

pub trait Injector<T, Infer> {
    fn inject(self) -> T;
}
