/// Types:
/// Type 1 - an adapter that returns a single constant, such as a u64
/// Type 2 - an adapter that returns a single value of the same type as the iterator
/// Type 3 - an adapter that returns another iterator.

// Step 1: Define a trait. The trait should extend Iterator so that if we
// pass a `LinqIteratorExtensions` trait object to a function it will have
// all the standard Iterator methods.
pub trait LinqIteratorExtensions : Iterator {
    fn single(&mut self) -> Option<Self::Item> {
        match self.next() {
            None => None,
            Some(x) => match self.next() {
                None => Some(x),
                Some(_) => None
            }
        }
    }

    fn single_or(&mut self, default: Self::Item) -> Self::Item {
        match self.next() {
            None => default,
            Some(x) => match self.next() {
                None => x,
                Some(_) => default
            }
        }
    }

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

    fn single_or_default(&mut self) -> Self::Item
        where Self::Item: Default
    {
        self.single_or(Default::default())
    }

//    fn in_range<T>(self, r: Range<T>) -> InRange<Self>
//        where Self: Iterator<Item = T> + Sized
//    {
//        InRange { iter: self, r, have_skipped: false }
//    }
}


// Step 2: Use that trait to add extension methods to the Iterator trait.
// The impl block itself is empty because we defaulted all the method definitions
// in the trait definition itself.
impl<T> LinqIteratorExtensions for T where T: Iterator { }


//// Step 3: Define the structs required by our adapters, if any: these are only
//// required if the adapters need to manage state.
//pub struct InRange<I>
//    where I: Iterator
//{
//    iter: I,
//    r: Range<I::Item>,
//    have_skipped: bool
//}
//
///// Step 4: The InRange struct is itself an iterator (it returns multiple items) so we need
///// to `impl Iterator` for it.
//impl<I> Iterator for InRange<I>
//    where I:Iterator,
//          I::Item: PartialOrd
//{
//    type Item = I::Item;
//
//    #[inline]
//    fn next(&mut self) -> Option<I::Item> {
//        if !self.have_skipped {
//            self.have_skipped = true;
//
//            while let Some(x) = self.iter.next() {
//                if x >= self.r.start {
//                    return Some(x);
//                }
//            }
//
//            return None;
//        }
//
//        if let Some(n) = self.iter.next() {
//            if n < self.r.end {
//                return Some(n);
//            }
//        }
//        None
//    }
//}

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
