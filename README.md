# Kinder
*Algebraic structure and emulation of higher-order types for Rust*

Kinder provides some tools and traits that functional programmers use daily.

***Kinder is very much a work in progress. The idea is to make HOT's approachable to rustaceans and provide a series of macros that make implementation of custom HOTs as painless as possible***.

**To Do**

1. ~~Finish implementation of Monad for std::collections~~

2. ~~Implement Applicative for std::collections~~

3. Implement Traverable for std::collections

4. Work on macros which make deriving these traits for custom types easy

5. Figure out how to implement Applicative for structs requiring Ord or Hash (the apply method is the issue, e.g what is an ordered function?)

**The Lift Module**

The lift module defines the Higher struct which allows creation of higher kinded types.
It also exports the macro lift! which implements Higher for types of kind * -> *.

**The SemiGroup Module**

Implements SemiGroup for std::collections as well as String.
Provides a method add which takes two items of the same type and returns an element of the same type.

**The Monoid Module**

Implements Monoid for std::collections as well as String.
Provides and id method for SemiGroups such that x.add(T::id()) = x.

**The Functor Module**

Implements Functor for std::collections and exports a macro functorize! which
makes a functor out of any lifted type which implements iter.

**The Applicative Module**

Implements Applicative for std::collections which supplies two methods.
Lift takes a T and raises it to be an A<T>, i.e Vec::lift(1) = vec!(1).
Apply takes an applicative, and a lifted functions and applies it, i.e vec!(1,2).apply(vec!(|x| x+1, |x| x*x)) = vec!(2, 4).

**The Monad Module**

Implements Monad for std::collections.
Monads have two functions, lift (normally return but return is reserved in Rust), and bind.
Lift takes and element and "lifts" it into the Monad, for example Option::lift(2) = Some(2).
Bind is similar to fmap except the mapping function has type: A -> M\<B> i.e i32 -> Option\<i32>.
Bind is often implemented using flat_map.

Example adding options:

```rust
extern crate kinder;
use kinder::lift::Monad;
fn add_option(x: &Option<i32>, y: i32) -> Option<i32> {
  x.bind(|elem| Some(elem+y))
}

fn add_options(x: &Option<i32>, y: &Option<i32>) -> Option<i32> {
  x.bind(|elem| add_option(y, *elem))
}
```

Example generic square function, credit to /u/stevenportzer on reddit for debugging and making the types work,
run with 
```bash
cargo run --example func-example
```

```rust 
extern crate kinder;
use kinder::lift::Functor;
use std::ops::Mul;

fn squares<A: Mul<Output=A> + Clone, T: Functor<A, B=A, C=T>>(xs: &T) -> T {
  xs.fmap(|&x| x*x)
}

fn main() {
  prinln!("{:?}", squares(&vec!(1,2,3)));  //will print [1, 4, 9]
}
```

![alt text](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7a455b42774743.57da548c501ce.gif "Rustaceans")

[Logo source](https://www.behance.net/gallery/42774743/Rustacean)

