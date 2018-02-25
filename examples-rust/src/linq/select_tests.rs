mod tests {
    #[test]
    fn select1_ints() {
        let source = vec![10, 20, 30, 40];
        let result : Vec<String> = source.iter().map(|x| format!("Hello {}", x)).collect();
        assert_eq!(result, vec!["Hello 10", "Hello 20", "Hello 30", "Hello 40"]);
    }

    #[test]
    fn select1_strings() {
        let source = vec!["red", "green", "blue", "white", "yellow"];
        let result : Vec<usize> = source.iter().map(|x| x.len()).collect();
        assert_eq!(result, vec![3, 5, 4, 5, 6]);
    }

    #[test]
    fn select2_ints() {
        let source = vec![10, 20, 30, 40];
        let result : Vec<String> = source.iter().enumerate()
            .map(|(idx, x)| format!("Hello {}", idx * x))
            .collect();

        assert_eq!(result, vec!["Hello 0", "Hello 20", "Hello 60", "Hello 120"]);
    }

    #[test]
    fn select2_strings() {
        let source = vec!["red", "green", "blue", "white", "yellow"];

        // Let's make a tuple this time.
        let result : Vec<(String, usize)> = source.iter().enumerate()
            .map(|(idx, x)| (format!("Hello {}", x), idx))
            .collect();

        assert_eq!(result, vec![
            ("Hello red".to_string(),    0),
            ("Hello green".to_string(),  1),
            ("Hello blue".to_string(),   2),
            ("Hello white".to_string(),  3),
            ("Hello yellow".to_string(), 4)
        ]);
    }
}