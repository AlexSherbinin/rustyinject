mod factory;
mod singleton;

pub use factory::*;
pub use singleton::*;

pub trait Injector<T, Idx, Strategy> {
    fn inject(self) -> T;
}
