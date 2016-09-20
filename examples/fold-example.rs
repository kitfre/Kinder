extern crate kinder;

use kinder::lift::{Foldable, Monoid, SemiGroup};

fn sum_foldable<B : SemiGroup<A=B> + Monoid<A=B>, T: Foldable<A=B>>(xs: &T) -> B 
{
    xs.foldr(B::id(), |x, y| x.add(y))
}

fn main() {
    let ints: Vec<i32> = vec!(1,2,3);
    let floats = vec!(1.0, 2.0, 3.0);
    let strings = vec!(String::from("Hello"), String::from(", "), String::from("World!"));

    println!("{}", sum_foldable(&ints)); //should print 6
    println!("{}", sum_foldable(&floats)); //should print 6.0
    println!("{}", sum_foldable(&strings)); //should print Hello, World!
}
