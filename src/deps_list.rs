use std::{convert::Infallible, marker::PhantomData};

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

pub trait DepsListGet<T, Idx> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl<Tail, T> DepsListGet<T, Last> for (T, Tail) {
    fn get(&self) -> &T {
        &self.0
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Head, Tail, T, Idx> DepsListGet<T, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListGet<T, Idx>,
{
    fn get(&self) -> &T {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.1.get_mut()
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
