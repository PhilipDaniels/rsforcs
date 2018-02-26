mod tests {
    use std::iter::{self, Empty};

    #[test]
    fn empty() {
        let mut empty_iter = iter::empty::<i32>();
        assert_eq!(None, empty_iter.next());
    }

    #[test]
    fn empty2() {
        let mut empty_iter : Empty<i32> = iter::empty();
        assert_eq!(None, empty_iter.next());
    }
}
