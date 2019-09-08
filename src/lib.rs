//! A simple crate for providing conversions from tuples to vectors and boxed
//! slices.
//!
//! # Small example
//!
//! This crate is pretty simple. Here's a trivial example.
//!
//! ```rust
//! # extern crate tuple_conv;
//! # use tuple_conv::RepeatedTuple;
//! let t = (0, 1, 2);
//! let v = t.to_vec();
//! assert_eq!(v, [0, 1, 2]);
//! ```
//!
//! # Motivation
//!
//! The primary motivation for using these tools is for syntactic elegance. In
//! APIs where a small, but variable number of arguments of a single type is
//! wanted, it's standard to use a `Vec<T>`. This can become cumbersome for the
//! user, however - particularly when we have nested types. See, for example:
//! ```
//! fn do_something_2d(a: Vec<Vec<i32>>) { /* ... */ }
//!
//! do_something_2d(vec![vec![1, 2, 3],
//!                      vec![4, 5, 6],
//!                      vec![7, 8, 9]]);
//! ```
//! Calling this function is somewhat cumbersome, and can be made cleaner with:
//! ```
//! # extern crate tuple_conv;
//! # use tuple_conv::RepeatedTuple;
//! fn do_something_2d<T, S>(a: T) where
//!     T: RepeatedTuple<S>,
//!     S: RepeatedTuple<i32>,
//! { /* ... */ }
//!
//! do_something_2d(((1, 2, 3),
//!                  (4, 5, 6),
//!                  (7, 8, 9)));
//! ```
//! Even though it starts to give us flashbacks from LISP, more of our code is
//! made up of things we actually care about - gone with the `vec` macros
//! everywhere. The primary benefit is simpler syntax.
//!
//! # Typical usage
//!
//! Although we can use [`RepeatedTuple`] as a type restriction, this would
//! usually be replacing a `Vec`, so there's a good chance we'd still like to
//! allow it. The main usage for this crate is then with the [`TupleOrVec`]
//! trait - it's implemented for all repeated tuples and for every `Vec`, which
//! allows us to easily change the public-facing API to allow tuples without
//! removing any functionality.
//!
//! Here's how we'd go about doing that, given a function `foo` that takes some
//! `Vec`:
//! ```
//! fn foo(v: Vec<&str>) {
//!     /* do stuff */
//! }
//!
//! // a typical way to call `foo`:
//! foo(vec!["bar", "baz"]);
//! ```
//! The first step is to change the function signature to accept tuples:
//! ```
//! extern crate tuple_conv;
//! use tuple_conv::TupleOrVec;
//!
//! // ...
//!
//! fn foo<V: TupleOrVec<&'static str>>(v: V) {
//!     /* do stuff */
//! }
//! ```
//! Then, convert the argument
//! ```
//! # extern crate tuple_conv;
//! # use tuple_conv::TupleOrVec;
//! fn foo<V: TupleOrVec<&'static str>>(v: V) {
//!     let v = v.as_vec();
//!     /* do stuff */
//! }
//! ```
//! And now we can call `foo` like this:
//! ```
//! # extern crate tuple_conv;
//! # use tuple_conv::TupleOrVec;
//! # fn foo<V: TupleOrVec<&'static str>>(v: V) {}
//! foo(("bar", "baz"));
//! foo(vec!["baz", "bar"]);
//! ```
//! It is, however, worth keeping in mind the implications of large generic
//! functions implemented for many types. This is discussed in more detail
//! [below](#performance).
//!
//! # Limitations and performance
//!
//! ### Limitations
//!
//! Because each new tuple is a distinct type, we can only implement for
//! finitely many tuple lengths. We've chosen to go up to tuples with 64
//! elements of the same type. If you find yourself needing more (although I
//! suspect this will be unlikely), the source is **relatively** simple, and
//! not too difficult to extend.
//!
//! Additionally, because of the Rust's visibility rules for public traits,
//! there isn't a good way to ensure that certain traits aren't implemented by
//! others - like [`TupleOrVec`] for example. That being said, the trait is
//! defined such that it *shouldn't* matter.
//!
//! ### Performance
//!
//! The details of the implementation are such that vectors are constructed in
//! reverse, and `Vec<_>.reverse()` called, due to a limitation of Rust's macro
//! system.
//!
//! This is not very significant (only ~10% increase with tuples of length 64),
//! but something worth considering for performance-critical code. For more
//! detail, pull this repository on [GitHub](add-link.com) and run
//! `cargo bench`.
//!
//! There are two other considerations: time to compile and final binary size.
//! These should usually be very minor - hardly noticeable. However: if you
//! are having issues, keep this in mind:  While these both may increase for
//! functions that are being implemented on many types, it may be possible to
//! reduce them by using public functions simply as wrappers for your internals
//! that only take vectors.
//!
//! [`RepeatedTuple`]: trait.RepeatedTuple.html
//! [`TupleOrVec`]: trait.TupleOrVec.html

/// A trait implemented on all tuples composed of a single type.[^1]
///
/// The available methods for this trait are what make up the standard way to
/// use this crate. Importing `RepeatedTuple` allows these methods to be called
/// directly on types you're working with. This trait can also be used loosely
/// as a bound specifically for repeated tuples, though there's nothing
/// stopping someone from implementing it on their own type.
///
/// A particularly nice use case of `RepeatedTuple` is ensuring a nice syntax
/// for your API. Because this is already discussed in the
/// [crate-level documentation], more examples will not be given here.
///
/// ### A few notes:
///
/// While this trait **can** be used as a trait bound, you may find it better
/// to instead use [`TupleOrVec`], as it also encapsulates vectors.
///
/// Additionally, while it is true in practice, there is no blanket
/// implementation of `TupleOrVec` for all `RepeatedTuple`, due to compiler
/// constraints.
///
/// Finally: The typical use case does not recommend or require re-implementing
/// this trait, but nothing will break if you do.
///
/// [^1]: Please note that this is only implemented for tuples up to size 64.
///     If you need more than 64, please fork this crate or submit a pull
///     request.
///
/// [crate-level documentation]: index.html
/// [`TupleOrVec`]: trait.TupleOrVec.html
pub trait RepeatedTuple<E>: Sized {
    /// Converts a tuple to a boxed slice, with elements in reverse order
    fn to_boxed_slice_reversed(self) -> Box<[E]>;

    /// Converts a tuple to a boxed slice of its elements
    fn to_boxed_slice(self) -> Box<[E]> {
        let mut s = self.to_boxed_slice_reversed();
        s.reverse();
        s
    }

    /// Converts a tuple to a vector, with elements in reverse order
    fn to_vec_reversed(self) -> Vec<E> {
        self.to_boxed_slice_reversed().into_vec()
    }

    /// Converts a tuple to a vector of its elements
    fn to_vec(self) -> Vec<E> {
        self.to_boxed_slice().into_vec()
    }
}

/// A trait implemented on repeated tuples and vectors.
///
/// This trait has already been covered in the [crate-level documentation], so
/// its coverage here will be brief.
///
/// This trait is implemented for all [repeated tuples] and all vectors, and
/// serves as a drop-in replacement for functions that take vectors as
/// arguments (albeit with some conversion). Types that were `Vec<T>` should be
/// converted to generic types that implement `TupleOrVec<T>`.
///
/// `TupleOrVec` is not designed with the intent of being implementable, but
/// there's nothing stopping you from doing so.
///
/// [crate-level documentation]: index.html
/// [repeated tuples]: trait.RepeatedTuple.html
pub trait TupleOrVec<E> {
    /// Converts the type to a vec
    fn as_vec(self) -> Vec<E>;
}

impl<E> TupleOrVec<E> for Vec<E> {
    fn as_vec(self) -> Vec<E> {
        self
    }
}

macro_rules! impl_tuple {
    (
        $E:ident,
        ($tup_head:ident, $($tup:ident),+),
        $idx_head:tt @ $($idx:tt)@+
    ) => {
        impl<$E> RepeatedTuple<$E> for ($tup_head, $($tup),+) {
            fn to_boxed_slice_reversed(self) -> Box<[$E]> {
                Box::new([self.$idx_head, $(self.$idx),+])
            }
        }

        impl<$E> TupleOrVec<$E> for ($tup_head, $($tup),+) {
            fn as_vec(self) -> Vec<$E> {
                RepeatedTuple::to_vec(self)
            }
        }

        impl_tuple! {
            $E,
            ($($tup),+),
            $($idx)@+
        }
    };

    // base case
    (
        $E:ident,
        ($tup:ident),
        $idx:tt
    ) => {
        impl<$E> RepeatedTuple<$E> for ($tup,) {
            fn to_boxed_slice_reversed(self) -> Box<[$E]> {
                Box::new([self.$idx])
            }
        }

        impl<$E> TupleOrVec<$E> for ($tup,) {
            fn as_vec(self) -> Vec<$E> {
                RepeatedTuple::to_vec(self)
            }
        }
    }
}

impl_tuple! {
    E,

    (E, E, E, E,
     E, E, E, E,
     E, E, E, E,
     E, E, E, E,

     E, E, E, E,
     E, E, E, E,
     E, E, E, E,
     E, E, E, E,

     E, E, E, E,
     E, E, E, E,
     E, E, E, E,
     E, E, E, E,

     E, E, E, E,
     E, E, E, E,
     E, E, E, E,
     E, E, E, E),


    63 @ 62 @ 61 @ 60 @
    59 @ 58 @ 57 @ 56 @
    55 @ 54 @ 53 @ 52 @
    51 @ 50 @ 49 @ 48 @

    47 @ 46 @ 45 @ 44 @
    43 @ 42 @ 41 @ 40 @
    39 @ 38 @ 37 @ 36 @
    35 @ 34 @ 33 @ 32 @

    31 @ 30 @ 29 @ 28 @
    27 @ 26 @ 25 @ 24 @
    23 @ 22 @ 21 @ 20 @
    19 @ 18 @ 17 @ 16 @

    15 @ 14 @ 13 @ 12 @
    11 @ 10 @  9 @  8 @
     7 @  6 @  5 @  4 @
     3 @  2 @  1 @  0
}

#[cfg(test)]
mod tests {
    use crate::RepeatedTuple;

    #[rustfmt::skip]
    macro_rules! long {
        (tuple) => {
            (
                1, 2, 3, 4, 5, 6, 7, 8,
                9, 10, 11, 12, 13, 14, 15, 16,
                17, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32,
                33, 34, 35, 36, 37, 38, 39, 40,
                41, 42, 43, 44, 45, 46, 47, 48,
                49, 50, 51, 52, 53, 54, 55, 56,
                57, 58, 59, 60, 61, 62, 63, 64,
            )
        };

        (slice_reversed) => {
            [
                64, 63, 62, 61, 60, 59, 58, 57,
                56, 55, 54, 53, 52, 51, 50, 49,
                48, 47, 46, 45, 44, 43, 42, 41,
                40, 39, 38, 37, 36, 35, 34, 33,
                32, 31, 30, 29, 28, 27, 26, 25,
                24, 23, 22, 21, 20, 19, 18, 17,
                16, 15, 14, 13, 12, 11, 10, 9,
                8, 7, 6, 5, 4, 3, 2, 1,
            ]
        };
    }

    #[test]
    fn to_boxed_slice_reversed() {
        let t = (1,);
        let b = t.to_boxed_slice_reversed();
        assert!(b == Box::new([1]));

        let t = (1, 2, 3);
        let b = t.to_boxed_slice_reversed();
        assert!(b == Box::new([3, 2, 1]));

        let t = long!(tuple);
        let b = t.to_boxed_slice_reversed();
        assert!(b == Box::new(long!(slice_reversed)));
    }

    #[test]
    fn to_boxed_slice() {
        let t = (1, 2, 3);
        let b = t.to_boxed_slice();
        assert!(b == Box::new([1, 2, 3]));
    }

    #[test]
    fn to_vec() {
        let t = (1, 2, 3);
        let v = t.to_vec();
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn to_vec_reversed() {
        let t = (1, 2, 3);
        let v = t.to_vec_reversed();
        assert_eq!(v, [3, 2, 1]);
    }
}
