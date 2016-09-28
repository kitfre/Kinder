use lift::{SemiGroup, Monoid};
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BTreeSet, HashSet, BinaryHeap};

//Implementation for numeric Monoids
monoid_num!(i8, 0);
monoid_num!(i16, 0);
monoid_num!(i32, 0);
monoid_num!(i64, 0);
monoid_num!(u8, 0);
monoid_num!(u16, 0);
monoid_num!(u32, 0);
monoid_num!(u64, 0);
monoid_num!(isize, 0);
monoid_num!(usize, 0);
monoid_num!(f32, 0.0);
monoid_num!(f64, 0.0);
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
