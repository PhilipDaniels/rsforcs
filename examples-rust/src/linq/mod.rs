
mod tests {
    #[test]
    fn where1_ints() {
        let source = vec![1, 2, 3, 4];

        // iter() yields &T. filter() takes &T and yields T.
        //
        // So in this very common example, iter() yields items of type &i32, then the closure to
        // filter() takes items of type &&i32 and returns items of type &i32.
        //
        // This is very different to C#, but makes sense when you realize that Rust always attempts
        // to avoid unnecessary copying, and if you want a copy you are made to ask for it. By
        // default you get a reference to the thing in the original collection (imagine if this was
        // a Vec<HugeStructure> rather than a Vec<int>).
        let result1 : Vec<&i32> = source.iter().filter(|&&x| x > 2).collect();
        assert_eq!(result1, vec![&3, &4]);

        // You can get rid of the references by using map().
        let result2 : Vec<i32> = source.iter().filter(|&&x| x > 2).map(|&x| x).collect();
        assert_eq!(result2, vec![3, 4]);

        // Or you can clone each item (assuming it supports the Clone trait). This is fine for
        // small values, but probably not what you want for large values. Note that we put the
        // cloned() call after the filter, so it has fewer items to process.
        // cloned turns an iterator over &T into an iterator over T by cloning each item.
        let result3 : Vec<i32> = source.iter().filter(|&&x| x > 2).cloned().collect();
        assert_eq!(result3, vec![3, 4]);

        // Finally, you can combine the filter and map calls using the filter_map method.
        let result4 : Vec<i32> = source.iter().filter_map(|&x|
            if x > 2 { Some(x) } else { None }
            ).collect();
        assert_eq!(result4, vec![3, 4]);

        // Just to emphasize, the original vector is unchanged.
        assert_eq!(source, vec![1, 2, 3, 4]);
    }

    #[test]
    fn where1_strings() {
        let source = vec!["red", "green", "blue", "white", "yellow"];
        let result : Vec<&str> = source.iter().filter_map(|&x|
            if x.contains("w") { Some(x) } else { None }
        ).collect();
        assert_eq!(result, vec!["white", "yellow"]);
    }

    #[test]
    fn where2_ints() {
        let source = vec![1, 2, 3, 4];
        let result1 : Vec<i32> = source.iter().enumerate()
            .filter(|&(idx, &x)| idx == 0 || x > 2)
            .map(|(_, &x)| x).collect();

        assert_eq!(result1, vec![1, 3, 4]);
    }

    #[test]
    fn where2_ints_filter_map() {
        let source = vec![1, 2, 3, 4];
        let result : Vec<i32> = source.iter().enumerate()
            .filter_map(|(idx, &x)| if idx == 0 || x > 2 { Some(x) } else { None })
            .collect();

        assert_eq!(result, vec![1, 3, 4]);
    }

    #[test]
    fn where2_strings() {
        let source = vec!["red", "green", "blue", "white", "yellow"];
        let result : Vec<&str> = source.iter().enumerate()
            .filter_map(|(idx, &x)| if idx == 0 || x.contains("w") { Some(x) } else { None })
            .collect();

        assert_eq!(result, vec!["red", "white", "yellow"]);
    }
}