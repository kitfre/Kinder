#![allow(dead_code, unused)]
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

///SemiGroup trait
///requires one function:
///add: &self -> &A -> A
pub trait SemiGroup {
    type A;
    fn add(&self, b: &Self::A) -> Self::A;
}

///monoid trait
///requires one function:
///id: &self -> A
pub trait Monoid : SemiGroup {
    fn id() -> Self::A;
}

/// functor trait, similar to Haskell's functor class
/// requires a function fmap of type: &self -> Fn(&Self::B) -> A
/// e.g Some(2).fmap(|x| x*x) = Some(4)
/// None.fmap(|x| x*x) = None
pub trait Functor<A>: Higher<A> {
    fn fmap<F>(&self, f: F) -> Self::C where F: Fn(&Self::B) -> A;
}

///applicative trait, similar to Haskell's applicative class
///requires two functions:
///lift (normally pure): lifts a B to an A<B> i.e Option::lift(2) = Some(2)
///apply (<*> in haskell): applies an applicative functor i.e Some(2).apply(Some(f)) => Some(f(2))
pub trait Applicative<A> : Higher<A> {
    fn lift(x: A) -> Self::C where Self: Higher<A, B=A>;
    fn apply<F>(&self, <Self as Higher<F>>::C) -> <Self as Higher<A>>::C where F: Fn(&<Self as Higher<A>>::B) -> A, Self: Higher<F>; //kinda ugly
}

/// monad trait, similar to Haskell's monad class
/// requires two functions:
/// lift (usually return but return is reserved): lifts an B to an A<B>, i.e Option::return(2) = Some(2)
/// bind: maps an A<B> to an A<C> i.e Some(2).bind(|x| Some(x+1)) = Some(3)
pub trait Monad<A>: Higher<A> {
    fn lift(x: A) -> Self::C where Self: Higher<A, B = A>;
    fn bind<F>(&self, F) -> Self::C where F: FnMut(&Self::B) -> Self::C;
}
