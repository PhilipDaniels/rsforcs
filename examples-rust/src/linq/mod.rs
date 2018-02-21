
mod tests {
    #[test]
    fn where1_ints() {
        let source = vec![1, 2, 3, 4];

        // iter() yields items as &i32, and filter takes things by reference, so it adds a second
        // &, resulting in &&x being needed to destructure them. filter then also yields items of
        // type &i32, so result1 is a Vec<&i32>. This is very different to C#, but makes sense
        // when you realize that Rust always attempts to avoid unnecessary copying, and if you
        // want a copy you are made to ask for it.
        let result1 : Vec<&i32> = source.iter().filter(|&&x| x > 2).collect();
        assert_eq!(result1, vec![&3, &4]);

        // What you probably want is a second vector containing just the items matching the
        // predicate. Do this by using the `cloned` function, which turns an iterator over &T into
        // an iterator over T by cloning each item (assuming T supports cloning). Integers do,
        // obviously. You could also do `map(|&x| x)` to dereference using a pattern match.
        let result2 : Vec<i32> = source.iter().cloned().filter(|&x| x > 2).collect();
        assert_eq!(result2, vec![3, 4]);

        // Just to emphasize, the original vector is unchanged.
        assert_eq!(source, vec![1, 2, 3, 4]);
    }

    #[test]
    fn where1_strings() {
        let source = vec!["red", "green", "blue", "white", "yellow"];
        let result : Vec<&str> = source.iter().filter(|&x| x.contains("w")).map(|&x| x).collect();
        assert_eq!(result, vec!["white", "yellow"]);
    }
}