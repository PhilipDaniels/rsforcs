mod tests {
    #[test]
    fn any_ints() {
        let source = vec![10, 20, 30, 40];
        assert_eq!(true, source.iter().any(|&x| x > 30));
        assert_eq!(false, source.iter().any(|&x| x < 0));
    }

    #[test]
    fn all_ints() {
        let source = vec![10, 20, 30, 40];
        assert_eq!(true, source.iter().all(|&x| x > 0));
        assert_eq!(false, source.iter().all(|&x| x % 3 == 0));
    }
}