mod tests {
    use std::iter::Iterator;

    /*
    struct Single<I> where I : Iterator {
        iter: I,
        item: Option<I::Item>
    }

    impl<I> Iterator for Single<I>
        where I: Iterator
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            match self.iter.next() {
                None => None,
                Some(x) => match self.iter.next() {
                    None => Some(x),
                    Some(_) => None
                }
            }
        }
    }

    trait SingleIteratorAdapter : Iterator {
        fn single(self) -> Single<Self>
            where Self: Sized
        {
            Single { iter: self, item: None }
        }
    }

    impl<I> SingleIteratorAdapter for I where I : Iterator { }
    */

    fn single_func<I: Iterator>(mut iter: I) -> Option<I::Item> {
        match iter.next() {
            None => None,
            Some(x) => match iter.next() {
                None => Some(x),
                Some(_) => None
            }
        }
    }

    #[test]
    fn single_ints_via_func() {
        let source = vec![10];
        let result = single_func(source.iter()).unwrap();
        assert_eq!(&10, result);

        let source : Vec<i32> = vec![];
        let result = single_func(source.iter());
        assert_eq!(None, result);

        let source = vec![10, 20];
        let result = single_func(source.iter());
        assert_eq!(None, result);
    }

    // #[test]
    // fn single_ints_via_iterator_adapter() {
    //     let source = vec![10];
    //     let result = source.iter().single().unwrap();
    //     assert_eq!(&10, result);

    //     let source : Vec<i32> = vec![];
    //     let result = source.iter().single();
    //     assert_eq!(None, result);

    //     let source = vec![10, 20];
    //     let result = source.iter().single();
    //     assert_eq!(None, result);
    // }
}