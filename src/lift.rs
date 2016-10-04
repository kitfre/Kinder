#![allow(dead_code, unused)]
#![feature(type_ascription)]
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

pub trait Higher<A> {
    type B; //current type inside higher type, i.e Vec<B>
    type C; //swapped higher type, i.e C = Vec<A>
}


/// macro to lift types
#[macro_export]
macro_rules! lift {
    ($t:ident) => {
        impl<A,C> Higher<A> for $t<C> {
            type B = C;
            type C = $t<A>;
        }
    }
}

// lifting types
lift!(Vec);
lift!(Option);
lift!(Box);
lift!(LinkedList);
lift!(BinaryHeap);
lift!(BTreeSet);
lift!(VecDeque);
lift!(HashSet);

///`SemiGroup` trait
///requires one function:
///add: &self -> &A -> A
pub trait SemiGroup {
    type A;
    fn add(&self, b: &Self::A) -> Self::A;
}

///`Monoid` trait
///requires one function:
///id: &self -> A
pub trait Monoid : SemiGroup {
    fn id() -> Self::A;
}

///`Foldable` trait
///requires an accumulator type `A`
///a function foldr: &self -> `Self::A` `f: F` -> `Self::A`
pub trait Foldable  {
    type A; //accumulator type
    fn foldr<F>(&self, accum: Self::A, f: F) -> Self::A
        where F: FnMut(Self::A, &Self::A) -> Self::A;
}

/// `Functor` trait, similar to Haskell's functor class
/// requires a function fmap of type: &self -> Fn(`&Self::B`) -> A
/// e.g `Some(2).fmap(|x| x*x) = Some(4)`
/// `None.fmap(|x| x*x) = None`
pub trait Functor<A>: Higher<A> {
    fn fmap<F>(&self, f: F) -> Self::C where F: Fn(&Self::B) -> A;
}

///`Applicative` trait, similar to Haskell's applicative class
///requires two functions:
///raise (normally pure): lifts a B to an A<B> i.e `Option::lift(2) = Some(2)`
///apply (<*> in haskell): applies an applicative functor i.e `Some(2).apply(Some(f)) = Some(f(2))`
pub trait Applicative<A> : Higher<A> {
    fn raise(x: A) -> Self::C where Self: Higher<A, B=A>;
    fn apply<F>(&self, <Self as Higher<F>>::C) -> <Self as Higher<A>>::C where F: Fn(&<Self as Higher<A>>::B) -> A, Self: Higher<F>; //kinda ugly
}

/// `Monad trait`, similar to Haskell's monad class
/// requires two functions:
/// lift (usually return but return is reserved): lifts an B to an A<B>, i.e `Option::lift(2) = Some(2)`
/// bind: maps an A<B> to an A<C> i.e `Some(2).bind(|x| Some(x+1)) = Some(3)`
pub trait Monad<A>: Higher<A> {
    fn lift(x: A) -> Self::C where Self: Higher<A, B = A>;
    fn bind<F>(&self, F) -> Self::C where F: FnMut(&Self::B) -> Self::C;
}

//macros
//
///A quick macro to functorize types implementing Iter
#[macro_export]
macro_rules! functorize {
    ($t:ident) => {
        impl<A,B> Functor<A> for $t<B> {
            fn fmap<F>(&self, f:F) -> $t<A> where F: Fn(&B) -> A {
                self.iter().map(f).collect::<$t<A>>()
            }
        }
    }
}

///A macro to implement monoid for numeric semigroups
#[macro_export]
macro_rules! monoid_num {
    ($t:ident, $z:expr) => {
        impl Monoid for $t {
            fn id() -> Self::A {
                $z
            }
        }
    }
}

///A macro to implement monoid for Semigroups which have a new method
#[macro_export]
macro_rules! monoid {
    ($t:ident) => {
        impl<T: Clone> Monoid for $t<T> {
            fn id() -> Self::A {
                $t::new()
            }
        }
    }
}

///Macro to implement ordered SemiGroups like BTreeSet which have a new method
#[macro_export]
macro_rules! monoid_ord {
    ($t:ident) => {
        impl<T: Clone + Ord> Monoid for $t<T> {
            fn id() -> Self::A {
                $t::new()
            }
        }
    }
}


///Macro to implement fold for iterables
#[macro_export]
macro_rules! foldable {
    ($t:ident) => {
        impl<T> Foldable for $t<T> {
            type A = T;
            fn foldr<F>(&self, accum: Self::A, f: F) -> Self::A
                where F: FnMut(Self::A, &Self::A) -> Self::A
            {
                self.iter().fold(accum, f)
            }
        }
    }
}

///Macro to implement semigroup for numerics
#[macro_export]
macro_rules! semigroup_num {
    ($t:ident) => {
        impl SemiGroup for $t {
            type A = $t;
            fn add(&self, b: &Self::A) -> Self::A {
                self + b
            }
        }
    }
}

///Macro for implementing SemiGroup for types that implement Extend
#[macro_export]
macro_rules! semigroup {
    ($t:ident) => {
        impl<T: Clone> SemiGroup for $t<T> {
            type A = $t<T>;
            fn add(&self, b: &Self::A) -> Self::A {
                let mut ret = $t::new();
                ret.extend(self.iter().cloned());
                ret.extend(b.iter().cloned());
                ret
            }
        }
    }
}

///A macro for implementing SemiGroups for types that need Ord and implement Extended
#[macro_export]
macro_rules! semigroup_ord {
    ($t:ident) => {
        impl<T: Clone + Ord> SemiGroup for $t<T> {
            type A = $t<T>;
            fn add(&self, b: &Self::A) -> Self::A {
                let mut ret = $t::new();
                ret.extend(self.iter().cloned());
                ret.extend(b.iter().cloned());
                ret
            }
        }
    }
}
