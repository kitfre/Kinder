extern crate kinder;
use kinder::lift::Monad;

fn add_option(x: &Option<i32>, y: i32) -> Option<i32> {
    x.bind(|elem| Some(elem+y))
}

fn add_options(x: &Option<i32>, y: &Option<i32>) -> Option<i32> {
    x.bind(|elem| add_option(y, *elem))
}

fn main() {
    println!("{:?}", add_option(&Some(1), 1));
    println!("{:?}", add_options(&Some(1), &Some(2)));
}
