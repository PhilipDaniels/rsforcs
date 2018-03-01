mod tests {
    #[test]
    fn first_ints() {
        // On an empty sequence, we get back a None.
        let source : Vec<i32> = vec![];
        assert_eq!(None, source.iter().next());

        // On a non-empty sequence, we get back a Option<&T> which we have to unwrap and then
        // de-reference using '*'. Admittedly, this is a bit verbose.
        let source = vec![10, 20, 30, 40];
        assert_eq!(10, *source.iter().next().unwrap());

        // The fact that we get back an Option leads us to an easy method of implementing the
        // `OrDefault` overloads, using the `unwrap_or*` functions on an Option. The most direct
        // equivalent uses `unwrap_or_default`, but this works only if the element type implements
        // the Default trait, and it probably won't do because by using `iter` you are getting
        // references to things rather than things. You can use `unwrap_or`, or `unwrap_or_else` to
        // calculate the default value using a closure.
        //
        // This is rather ugly, but at least we can specify our own default.
        // Dereference as above.
        let source : Vec<i32> = vec![];
        assert_eq!(&42, source.iter().next().unwrap_or(&42));
    }

    #[test]
    fn last_ints() {
        // On an empty sequence, we get back a None.
        let source : Vec<i32> = vec![];
        assert_eq!(None, source.iter().last());

        // On a non-empty sequence, we get back a Option<&T> which we have to unwrap and then
        // de-reference using '*'. Admittedly, this is a bit verbose.
        let source = vec![10, 20, 30, 40];
        assert_eq!(40, *source.iter().last().unwrap());

        // The fact that we get back an Option leads us to an easy method of implementing the
        // `OrDefault` overloads, using the `unwrap_or*` functions on an Option. The most direct
        // equivalent uses `unwrap_or_default`, but this works only if the element type implements
        // the Default trait, and it probably won't do because by using `iter` you are getting
        // references to things rather than things. You can use `unwrap_or`, or `unwrap_or_else` to
        // calculate the default value using a closure.
        //
        // This is rather ugly, but at least we can specify our own default.
        // Dereference as above.
        let source : Vec<i32> = vec![];
        assert_eq!(&42, source.iter().last().unwrap_or(&42));
    }

    trait FirstIteratorAdapter : Iterator {
        fn first_or_default(&mut self, default: Self::Item) -> Self::Item {
            self.next().unwrap_or(default)
        }
    }

    impl<I> FirstIteratorAdapter for I where I : Iterator { }

    #[test]
    fn first_ints_using_iterator_adapter() {
        // On an empty sequence, we get back a None.
        let source : Vec<i32> = vec![];
        assert_eq!(&42, source.iter().first_or_default(&42));

        // On a non-empty sequence, we get back a Option<&T> which we have to unwrap and then
        // de-reference using '*'. Admittedly, this is a bit verbose.
        let source = vec![10, 20, 30, 40];
        assert_eq!(10, *source.iter().first_or_default(&42));
    }
}