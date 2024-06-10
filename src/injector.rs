mod factory;
mod singleton;

use std::{convert::Infallible, marker::PhantomData};

pub use factory::*;
pub use singleton::*;

pub struct CurrentScope(Infallible);
pub struct ParentScope<Scope>(PhantomData<Scope>, Infallible);

pub trait Injector<T, Infer> {
    fn inject(self) -> T;
}
