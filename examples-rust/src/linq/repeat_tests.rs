mod tests {
    use std::iter;

    #[test]
    fn repeat_ints1() {
        let mut source = iter::repeat(4).take(2);
        assert_eq!(Some(4), source.next());
        assert_eq!(Some(4), source.next());
        assert_eq!(None, source.next());
    }

    #[test]
    fn repeat_ints2() {
        let source = iter::repeat(4).take(2);
        let result : Vec<i32> = source.collect();
        assert_eq!(result, vec![4, 4]);
    }

    #[test]
    fn repeat_string_slices() {
        let source = iter::repeat("hello").take(2);
        let result : Vec<&str> = source.collect();
        assert_eq!(result, vec!["hello", "hello"]);
    }
}