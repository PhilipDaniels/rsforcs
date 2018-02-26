mod tests {
    /// Non-consuming, multiple calls work fine.
    #[test]
    fn len_ints() {
        let source = vec![10, 20, 30, 40];
        let i = source.iter();
        assert_eq!(4, i.len());
        assert_eq!(4, i.len());
    }

    // Does not compile.
    // #[test]
    // fn count_ints() {
    //     let source = vec![10, 20, 30, 40];
    //     let i = source.iter();
    //     assert_eq!(4, i.count());
    //     assert_eq!(4, i.count());
    // }

    #[test]
    fn len_ints_with_predicate() {
        let source = vec![10, 20, 30, 40];
        let i = source.iter().filter(|&&x| x > 20);
        assert_eq!(2, i.count());
    }
}