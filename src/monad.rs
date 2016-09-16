#![allow(dead_code, unused)]
use lift::{Higher, Monad};
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::{BTreeSet, HashSet};

// Implementation of Monad for Vec
impl<A, B> Monad<A> for Vec<B> {
    fn lift(x: A) -> <Self as Higher<A>>::C {
        vec![x]
    }

    fn bind<F>(&self, mut f: F) -> Vec<A>
        where F: FnMut(&B) -> Vec<A>
    {
        self.iter().flat_map(f).collect::<Vec<A>>()
    }
}

// Implementation of Monad for Option
impl<A, B> Monad<A> for Option<B> {
    fn lift(x: A) -> <Self as Higher<A>>::C {
        Some(x)
    }

    fn bind<F>(&self, mut f: F) -> Option<A>
        where F: FnMut(&B) -> Option<A>
    {
        match *self {
            Some(ref v) => f(v),
            None => None,
        }
    }
}

//implementation of Moand for Box
impl<A,B> Monad<A> for Box<B> {
    fn lift(x:A) -> <Self as Higher<A>>::C {
        Box::new(x)
    }

    fn bind<F>(&self, mut f: F) -> Box<A> 
        where F : FnMut(&B) -> Box<A> 
    {
        f(self)
    }
}

// Implementation of Monad for LinkedList
impl<A, B> Monad<A> for LinkedList<B> {
    fn lift(x: A) -> <Self as Higher<A>>::C {
        let mut ret = LinkedList::new();
        ret.push_back(x);
        ret
    }

    fn bind<F>(&self, mut f: F) -> LinkedList<A>
        where F: FnMut(&B) -> LinkedList<A>
    {
        self.iter().flat_map(f).collect::<LinkedList<A>>()
    }
}

// Implementations of Monad for BTreeSet
impl<A: Ord, B: Ord> Monad<A> for BTreeSet<B> {
    fn lift(x: A) -> <Self as Higher<A>>::C {
        let mut ret = BTreeSet::new();
        ret.insert(x);
        ret
    }

    fn bind<F>(&self, f: F) -> BTreeSet<A>
        where F: FnMut(&B) -> BTreeSet<A>
    {
        self.iter().flat_map(f).collect::<BTreeSet<A>>()
    }
}

// Implementations of Monad for HashSet
impl<A: Eq + Hash, B: Eq + Hash> Monad<A> for HashSet<B> {
    fn lift(x: A) -> <Self as Higher<A>>::C {
        let mut ret = HashSet::new();
        ret.insert(x);
        ret
    }

    fn bind<F>(&self, f: F) -> HashSet<A>
        where F: FnMut(&B) -> HashSet<A>
    {
        self.iter().flat_map(f).collect::<HashSet<A>>()
    }
}

#[cfg(test)]
mod test {
    use lift::{Higher, Monad};
    
    #[test]
    fn test_option() {
        assert_eq!(Option::lift("a"), Some("a"));
        assert_eq!(Some(2).bind(|x| Some(x+1)), Some(3));
    }

    #[test]
    fn test_box() {
        assert_eq!(Box::lift(2), Box::new(2));
        assert_eq!(Box::new(2).bind(|x| Box::new(x+1)), Box::new(3));
    }

}
