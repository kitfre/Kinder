extern crate kinder;
use kinder::lift::{SemiGroup, Functor};

fn fmap_vec(vec: &Vec<i32>) -> Vec<i32> {
    vec.fmap(|x| x*x) //square all elements in a vec<i32>
}

fn append_vec(vec: &Vec<i32>, two: &Vec<i32>) -> Vec<i32> {
    vec.add(two)
}

fn main() {
    let vec = vec!(1,2,3);
    let two = vec!(4,5,6);

    println!("{:?}", fmap_vec(&vec));
    println!("{:?}", append_vec(&vec, &two));
}
