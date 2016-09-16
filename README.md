# Kinder
*Algebraic structure and emulation of higher kinded types for Rust*

Kinder provides some tools and traits that functional programmers use daily.

| *Kinder is very much a work in progress. The idea is to make HKT's approachable to rustaceans and provide a series of macros that make implementation of custom HKTs as painless as possible*.|

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

**The Monad Module**
Implements Monad for std::collections.
Monads have two functions, lift (normally return but return is reserved in Rust), and bind.
Lift takes and element and "lifts" it into the Monad, for example Option::lift(2) = Some(2).
Bind is similar to fmap except the mapping function has type: A -> M(B) i.e i32 -> Option<i32>.
Bind is often implemented using flat_map.
