mod tests {
    #[test]
    fn chain_ints() {
        // Using into_iter() to avoid having to call filter_map to
        // get a Vec<i32> instead of a Vec<&i32>.
        let v1 = vec![10, 20].into_iter();
        let v2 = vec![30, 40].into_iter();
        let result : Vec<i32> = v1.chain(v2).collect();
        assert_eq!(result, vec![10, 20, 30, 40]);
    }
}