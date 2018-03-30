/// Types:
/// Type 1 - an adapter that returns a single constant, such as a u64
/// Type 2 - an adapter that returns a single value of the same type as the iterator
/// Type 3 - an adapter that returns another iterator.

use std::hash;
use std::collections::HashSet;

// Step 1: Define a trait. The trait should extend Iterator so that if we
// pass a `LinqIteratorExtensions` trait object to a function it will have
// all the standard Iterator methods.
pub trait LinqIteratorExtensions : Iterator {
    #[inline]
    fn single(&mut self) -> Option<Self::Item> {
        match self.next() {
            None => None,
            Some(x) => match self.next() {
                None => Some(x),
                Some(_) => None
            }
        }
    }

    #[inline]
    fn single_or(&mut self, default: Self::Item) -> Self::Item {
        match self.next() {
            None => default,
            Some(x) => match self.next() {
                None => x,
                Some(_) => default
            }
        }
    }

    #[inline]
    fn single_or_else<F>(&mut self, f: F) -> Self::Item
        where F: FnOnce() -> Self::Item
    {
        // We could change this to `self.single_or(f())`, but that would mean the function
        // was always called, whether or not it needed to be.
        match self.next() {
            None => f(),
            Some(x) => match self.next() {
                None => x,
                Some(_) => f()
            }
        }
    }

    #[inline]
    fn single_or_default(&mut self) -> Self::Item
        where Self::Item: Default
    {
        self.single_or(Default::default())
    }

    #[inline]
    fn first_or(&mut self, default: Self::Item) -> Self::Item {
        self.next().unwrap_or(default)
    }

    #[inline]
    fn first_or_else<F>(&mut self, f: F) -> Self::Item
        where F: FnOnce() -> Self::Item
    {
        self.next().unwrap_or_else(f)
    }

    #[inline]
    fn first_or_default(&mut self) -> Self::Item
        where Self::Item: Default
    {
        self.first_or(Default::default())
    }


    // Here we get into adapters that need a struct.

    // Because we use IntoIterator we can pass anything that can be converted into an Iterator,
    // not just an actual Iterator. For example slices `&[T]`.
    #[inline]
    fn intersect<U>(self, other: U) -> Intersect<Self, U::IntoIter>
        where Self: Sized,
              Self::Item: hash::Hash + Eq,
              U: IntoIterator<Item = Self::Item>
    {
        Intersect { a: self, b: other.into_iter(), items: HashSet::new() }
    }
}


// Step 2: Use that trait to add extension methods to the Iterator trait.
// The impl block itself is empty because we defaulted all the method definitions
// in the trait definition itself.
impl<T> LinqIteratorExtensions for T where T: Iterator { }


// Step 3: Define the structs required by our adapters, if any: these are only required if the
// adapters need to manage state. n.b. Step 3 implies you need step 4 as well.
pub struct Intersect<A, B>
    where A: Iterator,
          B: Iterator
{
    a: A,
    b: B,
    items: HashSet<A::Item>
}

// Step 4: Implement Iterator functionality for our structs.
impl<A, B> Iterator for Intersect<A, B>
    where A: Iterator,
          B: Iterator<Item = A::Item>,
          A::Item: Eq + hash::Hash
{
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> {
        //self.items = self.b.collect();
        //self.b
        None
    }
}

#[cfg(test)]
mod tests {
    use super::LinqIteratorExtensions;
    use std::iter::{empty, once};

    #[test]
    fn single_for_empty_sequence_returns_none() {
        let actual = empty::<i32>().single();
        assert_eq!(actual, None);
    }

    #[test]
    fn single_for_singleton_sequence_returns_none() {
        let actual = once(12).single();
        assert_eq!(actual, Some(12));
    }

    #[test]
    fn single_for_sequence_of_length_two_or_more_returns_none() {
        let actual = (0..2).single();
        assert_eq!(actual, None);

        let actual = (0..3).single();
        assert_eq!(actual, None);
    }

    #[test]
    fn single_or_for_empty_sequence_returns_default() {
        let actual = empty::<i32>().single_or(42);
        assert_eq!(actual, 42);
    }

    #[test]
    fn single_or_for_singleton_sequence_returns_value_from_sequence() {
        let actual = once(12).single_or(42);
        assert_eq!(actual, 12);
    }

    #[test]
    fn single_or_for_sequence_of_length_two_or_more_returns_default() {
        let actual = (0..2).single_or(42);
        assert_eq!(actual, 42);

        let actual = (0..3).single_or(42);
        assert_eq!(actual, 42);
    }

    fn default_generator() -> i32 {
        130
    }

    #[test]
    fn single_or_else_for_empty_sequence_returns_default() {
        let actual = empty::<i32>().single_or_else(default_generator);
        assert_eq!(actual, 130);
    }

    #[test]
    fn single_or_else_for_singleton_sequence_returns_value_from_sequence() {
        let actual = once(12).single_or_else(default_generator);
        assert_eq!(actual, 12);
    }

    #[test]
    fn single_or_else_for_sequence_of_length_two_or_more_returns_default() {
        let actual = (0..2).single_or_else(default_generator);
        assert_eq!(actual, 130);

        let actual = (0..3).single_or_else(default_generator);
        assert_eq!(actual, 130);
    }

    #[test]
    fn single_or_default_for_empty_sequence_returns_default() {
        let actual = empty::<i32>().single_or_default();
        assert_eq!(actual, 0);
    }

    #[test]
    fn single_or_default_for_singleton_sequence_returns_value_from_sequence() {
        let actual = once(12).single_or_default();
        assert_eq!(actual, 12);
    }

    #[test]
    fn single_or_default_for_sequence_of_length_two_or_more_returns_default() {
        let actual = (0..2).single_or_default();
        assert_eq!(actual, 0);

        let actual = (0..3).single_or_default();
        assert_eq!(actual, 0);
    }

    #[test]
    fn first_or_for_empty_sequence_returns_default() {
        assert_eq!(empty::<i32>().first_or(42), 42);
    }

    #[test]
    fn first_or_for_singleton_sequence_returns_first_value_from_sequence() {
        assert_eq!(once(12).first_or(42), 12);
    }

    #[test]
    fn first_or_for_sequence_of_length_two_or_more_returns_first_value_from_sequence() {
        let actual = (1..3).first_or(42);
        assert_eq!(actual, 1);

        let actual = (1..4).first_or(42);
        assert_eq!(actual, 1);
    }

    #[test]
    fn first_or_else_for_empty_sequence_returns_default() {
        assert_eq!(empty::<i32>().first_or_else(default_generator), 130);
    }

    #[test]
    fn first_or_else_for_singleton_sequence_returns_first_value_from_sequence() {
        assert_eq!(once(12).first_or_else(default_generator), 12);
    }

    #[test]
    fn first_or_else_for_sequence_of_length_two_or_more_returns_first_value_from_sequence() {
        let actual = (1..3).first_or_else(default_generator);
        assert_eq!(actual, 1);

        let actual = (1..4).first_or_else(default_generator);
        assert_eq!(actual, 1);
    }

    #[test]
    fn first_or_default_for_empty_sequence_returns_default() {
        assert_eq!(empty::<i32>().first_or_default(), 0);
    }

    #[test]
    fn first_or_default_for_singleton_sequence_returns_first_value_from_sequence() {
        assert_eq!(once(12).first_or_default(), 12);
    }

    #[test]
    fn first_or_default_for_sequence_of_length_two_or_more_returns_first_value_from_sequence() {
        let actual = (1..3).first_or_default();
        assert_eq!(actual, 1);

        let actual = (1..4).first_or_default();
        assert_eq!(actual, 1);
    }


//    #[test]
//    fn in_range_works() {
//        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let result = v.into_iter().in_range(0..3).collect::<Vec<_>>();
//        assert_eq!(result, vec![0, 1, 2]);
//
//        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let result = v.into_iter().in_range(4..8).collect::<Vec<_>>();
//        assert_eq!(result, vec![4, 5, 6, 7]);
//
//        // TODO: This is ugly...
//        let v= vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let result = v.iter().in_range(&4..&8).collect::<Vec<_>>();
//        assert_eq!(result, vec![&4, &5, &6, &7]);
//    }
}
