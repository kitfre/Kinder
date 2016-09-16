#![allow(dead_code, unused)]
use lift::{SemiGroup, Monoid};
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BTreeSet, HashSet, BinaryHeap};


//macro to implement monoid for SemiGroups like Vec which have a new method
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

//macro to implement monoid for ordered SemiGroups like BTreeSet which have a new method
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

//Implementation of Monoid for String
impl Monoid for String {
    fn id() -> Self::A {
        String::from("")
    }
}

//Implementation of Monoid for HashSet
impl<T: Hash + Clone + Eq> Monoid for HashSet<T> {
    fn id() -> Self::A {
        HashSet::new()
    }
}

//Implementation of Monoid for Vec<T>
monoid!(Vec);

//Implementation of Monoid for LinkedList<T>
monoid!(LinkedList);

//Implementation of Monoid for VecDeque<T>
monoid!(VecDeque);

//Implementation of Monoid for BinaryHeap<T>
monoid_ord!(BinaryHeap);

//Implementation of Monoid for BTreeSet<T> 
monoid_ord!(BTreeSet);
