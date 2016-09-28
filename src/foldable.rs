use lift::Foldable;
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

//Implementation of Foldable for Vec
foldable!(Vec);

//Implementation of Foldable for LinkedList
foldable!(LinkedList);

//Implementation of Foldable for VeqDeque
foldable!(VecDeque);

//Implemenatation of Foldable for BinaryHeap
impl<T: Ord> Foldable for BinaryHeap<T> {
    type A = T;
    fn foldr<F>(&self, accum: Self::A, f: F) -> Self::A
        where F: FnMut(Self::A, &Self::A) -> Self::A
    {
        self.iter().fold(accum, f)
    }
}

//Implementation of Foldable for BTreeSet
impl<T: Ord> Foldable for BTreeSet<T> {
    type A = T;
    fn foldr<F>(&self, accum: Self::A, f: F) -> Self::A
        where F: FnMut(Self::A, &Self::A) -> Self::A
    {
        self.iter().fold(accum, f)
    }
}

//Implementation of Foldable for HashSet
impl<T: Hash + Eq> Foldable for HashSet<T> {
    type A = T;
    fn foldr<F>(&self, accum: Self::A, f: F) -> Self::A
        where F: FnMut(Self::A, &Self::A) -> Self::A
    {
        self.iter().fold(accum, f)
    }
}

