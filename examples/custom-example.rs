#![allow(dead_code, unused)]

#[macro_use]
extern crate kinder;
use kinder::lift::*;

//We'll define a very simply custom struct and then implement HOT's for it
#[derive(Debug)] //for printing nicely
struct Holder<T> {
    elem: T, //simply holds a value of type T
}

//now we lift it to a higher type
lift!(Holder);

impl<A,B> Functor<A> for Holder<B> {
    fn fmap<F>(&self, f: F) -> Holder<A> 
        where F: Fn(&B) -> A
    {
        Holder{ elem: f(&self.elem) } //just applies f to elem
    }
}

impl<A, B> Monad<A> for Holder<B> {
    fn lift(x:A) -> <Self as Higher<A>>::C {
        Holder { elem: x }
    }

    fn bind<F>(&self, mut f: F) -> Holder<A>
        where F: FnMut(&B) -> Holder<A>
    {
        f(&self.elem) //apply f to elem -> returns a holder
    }
}


fn main() {
    let test = Holder::lift(2);
    println!("{:?}", test); //prints Holder { elem: 2 }
    
    let mapped = test.fmap(|x| x+4);
    println!("{:?}", mapped); //prints Holder { elem: 6 }

}
