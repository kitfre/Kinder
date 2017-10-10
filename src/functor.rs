use lift::{HigherOrder, Functor};
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

// Implementation of Functor for Option
impl<A, B> Functor<A> for Option<B> {
    fn fmap<F>(&self, f: F) -> Option<A>
        where F: Fn(&B) -> A
    {
        match *self {
            Some(ref x) => Some(f(x)),
            None => None,
        }
    }
}

// Implementation of Functor for Box
impl<A, B> Functor<A> for Box<B> {
    fn fmap<F>(&self, f: F) -> Box<A>
        where F: Fn(&B) -> A
    {
        Box::new(f(self))
    }
}

// Implementation of Functor for BinaryHeap
impl<A: Ord, B: Ord> Functor<A> for BinaryHeap<B> {
    fn fmap<F>(&self, f: F) -> BinaryHeap<A>
        where F: Fn(&B) -> A
    {
        self.iter().map(f).collect::<BinaryHeap<A>>()
    }
}

// Implementation of Functor for BTreeSet
impl<A: Ord, B: Ord> Functor<A> for BTreeSet<B> {
    fn fmap<F>(&self, f: F) -> BTreeSet<A>
        where F: Fn(&B) -> A
    {
        self.iter().map(f).collect::<BTreeSet<A>>()
    }
}

// Implementation of Functor for HashSet
impl<A: Eq + Hash, B: Eq + Hash> Functor<A> for HashSet<B> {
    fn fmap<F>(&self, f: F) -> HashSet<A>
        where F: Fn(&B) -> A
    {
        self.iter().map(f).collect::<HashSet<A>>()
    }
}

// Implementation of Functor for LinkedList
functorize!(LinkedList);

// Implementation of Functor for Vec
functorize!(Vec);

// Implementation of Functor for VecDeque
functorize!(VecDeque);

// Tests of each HKT functor
#[cfg(test)]
mod test {
    use lift::Functor;

    #[test]
    fn test_option() {
        assert_eq!(Some(2).fmap(|x| x + 1), Some(3));
    }

    #[test]
    fn test_vec() {
        assert_eq!(vec![1, 2, 3].fmap(|x| x * x), vec![1, 4, 9]);
    }

    #[test]
    fn test_box() {
        assert_eq!(Box::new(1).fmap(|x| x + 1), Box::new(2));
    }
}
