#![allow(dead_code, unused)]

#[macro_use] extern crate kinder;
#[macro_use] extern crate kinder_derive;
use kinder::lift::*;

//We'll define a very simply custom struct and then implement HOT's for it
#[derive(Debug, HigherOrder)]
struct Holder<T> {
    elem: T, //simply holds a value of type T
}

#[derive(Debug, HigherOrder, Functor)]
enum Maybe<T> {
    Just(Vec<T>),
    Nothing
}

impl<A,B> Functor<A> for Holder<B> {
    fn fmap<F>(&self, f: F) -> Holder<A> 
        where F: Fn(&B) -> A
    {
        Holder{ elem: f(&self.elem) } //just applies f to elem
    }
}

impl<A, B> Monad<A> for Holder<B> {
    fn lift(x:A) -> <Self as HigherOrder<A>>::C {
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

    let test2 = Maybe::Just(vec!(1, 2)); 
    println!("{:?}", test2); // prints Just([1, 2])

    let mapped2 = test2.fmap(|x| x + 4);
    println!("{:?}", mapped2); // prints Just([5, 6])

}
