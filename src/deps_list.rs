use std::{
    convert::Infallible,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

pub trait DepsList: Sized {
    type PrependedWith<T>;

    fn prepend<T>(self, value: T) -> Self::PrependedWith<T>;
}

impl DepsList for () {
    type PrependedWith<T> = (T, Self);

    fn prepend<T>(self, value: T) -> Self::PrependedWith<T> {
        (value, self)
    }
}

impl<Head, Tail> DepsList for (Head, Tail) {
    type PrependedWith<T> = (T, Self);

    fn prepend<T>(self, value: T) -> Self::PrependedWith<T> {
        (value, self)
    }
}

pub struct Last(Infallible);
pub struct Next<Idx>(PhantomData<Idx>, Infallible);

pub trait DepsListGetRef<T, Idx> {
    fn get(&self) -> &T;
}

pub trait DepsListGetMut<T, Idx> {
    fn get_mut(&mut self) -> &mut T;
}

impl<Tail, T> DepsListGetRef<T, Last> for (T, Tail) {
    fn get(&self) -> &T {
        &self.0
    }
}

impl<Head, Tail, T, Idx> DepsListGetRef<T, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        self.1.get()
    }
}

impl<D, T, Idx> DepsListGetRef<T, Idx> for &D
where
    D: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        (*self).get()
    }
}

impl<D, T, Idx> DepsListGetRef<T, Idx> for Box<D>
where
    D: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        self.deref().get()
    }
}

impl<D, T, Idx> DepsListGetRef<T, Idx> for Rc<D>
where
    D: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        self.deref().get()
    }
}

impl<D, T, Idx> DepsListGetRef<T, Idx> for Arc<D>
where
    D: DepsListGetRef<T, Idx>,
{
    fn get(&self) -> &T {
        self.deref().get()
    }
}

impl<Tail, T> DepsListGetMut<T, Last> for (T, Tail) {
    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Head, Tail, T, Idx> DepsListGetMut<T, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListGetMut<T, Idx>,
{
    fn get_mut(&mut self) -> &mut T {
        self.1.get_mut()
    }
}

impl<D, T, Idx> DepsListGetMut<T, Idx> for &mut D
where
    D: DepsListGetMut<T, Idx>,
{
    fn get_mut(&mut self) -> &mut T {
        (*self).get_mut()
    }
}

impl<D, T, Idx> DepsListGetMut<T, Idx> for Box<D>
where
    D: DepsListGetMut<T, Idx>,
{
    fn get_mut(&mut self) -> &mut T {
        self.deref_mut().get_mut()
    }
}

pub trait DepsListRemove<T, Idx> {
    type Remainder;

    fn remove(self) -> (T, Self::Remainder);
}

impl<Tail, T> DepsListRemove<T, Last> for (T, Tail) {
    type Remainder = Tail;

    fn remove(self) -> (T, Self::Remainder) {
        self
    }
}

impl<Head, Tail, T, Idx> DepsListRemove<T, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListRemove<T, Idx>,
{
    type Remainder = (Head, Tail::Remainder);

    fn remove(self) -> (T, Self::Remainder) {
        let (removed, tail_remainder) = self.1.remove();
        (removed, (self.0, tail_remainder))
    }
}
