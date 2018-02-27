mod tests {
    use std::iter::Iterator;

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



    #[test]
    fn single_ints() {
        let source = vec![10];
        let result = source.iter().single().item.unwrap();
        assert_eq!(&10, result);
    }

    // #[test]
    // fn all_ints() {
    //     let source = vec![10, 20, 30, 40];
    //     assert_eq!(true, source.iter().all(|&x| x > 0));
    //     assert_eq!(false, source.iter().all(|&x| x % 3 == 0));

    // }
}