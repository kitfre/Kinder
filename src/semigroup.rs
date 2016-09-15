#![allow(dead_code, unused)]
use lift::{SemiGroup};
use std::collections::linked_list::LinkedList;

//Implementataion of Monoid for String
impl SemiGroup for String {
    type A = String;

    fn add(&self, b: &Self::A) -> Self::A {
        let mut ret = String::from("");
        ret.push_str(&self);
        ret.push_str(&b);
        ret
    }
}

//Implementation of SemiGroup for Vec<T>
impl<T: Clone> SemiGroup for Vec<T> {
    type A = Vec<T>;

    fn add(&self, b: &Self::A) -> Self::A {
        let mut ret = Vec::new();
        ret.extend(self.iter().cloned());
        ret.extend(b.iter().cloned());
        ret
    }
}



#[cfg(test)]
mod test {
    use lift::{SemiGroup};
    
    #[test]
    fn test_vec() {
        let one = vec!(1,2);
        let two = vec!(3,4);

        assert_eq!(one.add(&two), vec!(1,2,3,4));
        assert_eq!(one, vec!(1,2));
    }
    
    #[test]
    fn test_string() {
        let one = String::from("one");
        let two = String::from("two");

        assert_eq!(one.add(&two), String::from("onetwo")); 
    }
}

