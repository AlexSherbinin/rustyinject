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

pub trait DepsListGet<Predicate, Idx> {
    type Value;

    fn get(&self) -> &Self::Value;
    fn get_mut(&mut self) -> &mut Self::Value;
}

pub trait PredicateMatches<T> {}

impl<Head, Tail, Predicate> DepsListGet<Predicate, Last> for (Head, Tail)
where
    Predicate: PredicateMatches<Head>,
{
    type Value = Head;

    fn get(&self) -> &Self::Value {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Self::Value {
        &mut self.0
    }
}

impl<Head, Tail, Predicate, Idx> DepsListGet<Predicate, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListGet<Predicate, Idx>,
{
    type Value = Tail::Value;

    fn get(&self) -> &Self::Value {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut Self::Value {
        self.1.get_mut()
    }
}

pub trait DepsListRemove<Predicate, Idx> {
    type Removed;
    type Remainder;

    fn remove(self) -> (Self::Removed, Self::Remainder);
}

impl<Head, Tail, Predicate> DepsListRemove<Predicate, Last> for (Head, Tail)
where
    Predicate: PredicateMatches<Head>,
{
    type Removed = Head;
    type Remainder = Tail;

    fn remove(self) -> (Self::Removed, Self::Remainder) {
        self
    }
}

impl<Head, Tail, Predicate, Idx> DepsListRemove<Predicate, Next<Idx>> for (Head, Tail)
where
    Tail: DepsListRemove<Predicate, Idx>,
{
    type Removed = Tail::Removed;
    type Remainder = (Head, Tail::Remainder);

    fn remove(self) -> (Self::Removed, Self::Remainder) {
        let (removed, tail_remainder) = self.1.remove();
        (removed, (self.0, tail_remainder))
    }
}
