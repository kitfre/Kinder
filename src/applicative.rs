use lift::{Higher, Applicative};
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;

//Implementation of Applicative for Option
impl<A,B> Applicative<A> for Option<B> {
    fn raise(x:A) -> <Self as Higher<A>>::C {
        Some(x)
    }

    fn apply<F>(&self, f: <Self as Higher<F>>::C) -> <Self as Higher<A>>::C 
        where F: Fn(&<Self as Higher<A>>::B) -> A
    {
        match *self {
            Some(ref v) => match f {
                Some(fs) => Some(fs(v)),
                None => None,
            },
            None => None,          
        }
    }
}

//Implementation of Applicative for Vec
impl<A,B> Applicative<A> for Vec<B> {
    fn raise(x:A) -> <Self as Higher<A>>::C {
        vec!(x)
    }

    fn apply<F>(&self, fs: <Self as Higher<F>>::C) -> <Self as Higher<A>>::C
        where F: Fn(&<Self as Higher<A>>::B) -> A
    {
        let zipped = self.iter().zip(fs.iter()); //vec!(1,2,3).iter().zip(vec!(f1,f2,f3)) => zip((1,f1), (2,f2), (3,f3))
        zipped.map(|(x,f)| f(x)).collect::<Vec<A>>()
    }
}

//Implementation of Applicative for LinkedList
impl<A,B> Applicative<A> for LinkedList<B> {
    fn raise(x:A) -> <Self as Higher<A>>::C {
        let mut ret = LinkedList::new();
        ret.push_back(x);
        ret
    }

    fn apply<F>(&self, fs:<Self as Higher<F>>::C) -> <Self as Higher<A>>::C 
        where F: Fn(&<Self as Higher<A>>::B) -> A
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x,f)| f(x)).collect::<LinkedList<A>>()
    }
}

//Implemenation of Applicative for Box
impl<A,B> Applicative<A> for Box<B> {
    fn raise(x:A) -> <Self as Higher<A>>::C {
        Box::new(x)
    }

    fn apply<F>(&self, fs: <Self as Higher<F>>::C) -> <Self as Higher<A>>::C 
        where F: Fn(&<Self as Higher<A>>::B) -> A
    {
        Box::new(fs(self))
    }
}

//Implementation of Applicative for VecDeque
impl<A,B> Applicative<A> for VecDeque<B> {
    fn raise(x:A) -> <Self as Higher<A>>::C {
        let mut ret = VecDeque::new();
        ret.push_back(x);
        ret
    }

    fn apply<F>(&self, fs: <Self as Higher<F>>::C) -> <Self as Higher<A>>::C 
        where F: Fn(&<Self as Higher<A>>::B) -> A
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x,f)| f(x)).collect::<VecDeque<A>>()
    }
}

