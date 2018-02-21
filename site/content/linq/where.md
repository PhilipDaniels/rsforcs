+++
title = "Where"
weight = 100
sort_by = "weight"
+++

## Overloads

[Where](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.where?view=netframework-4.7.1#System_Linq_Enumerable_Where)
is used to filter a sequence based on a predicate. There are two overloads of the `Where` method,
the first one passes each item to the predicate, the second overload passes the item and the index
of the item:

```cs
1: public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
2: public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, int, bool> predicate);
```

## First overload

The Rust equivalent of `Where` is [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter):

```cs
[Test]
public void Where1_Ints()
{
    var source = new List<int> { 1, 2, 3, 4 };
    var result = source.Where(x => x > 2).ToList();
    CollectionAssert.AreEqual(result, new List<int> { 3, 4 });
}

[Test]
public void Where1_Strings()
{
    var source = new List<string> { "red", "green", "blue", "white", "yellow" };
    var result = source.Where(x => x.Contains("w")).ToList();
    CollectionAssert.AreEqual(result, new List<String> { "white", "yellow" });
}
```

```rs
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
    // obviously. However, there is a better way...
    let result2 : Vec<i32> = source.iter().cloned().filter(|&x| x > 2).collect();
    assert_eq!(result2, vec![3, 4]);

    // You can also do `map(|&x| x)` to dereference using a pattern match.
    // Since this occurs after the filter, it has to clone fewer elements.
    let result3 : Vec<i32> = source.iter().filter(|&x| x > 2).map(|&x| x).collect();
    assert_eq!(result3, vec![3, 4]);

    // Just to emphasize, the original vector is unchanged.
    assert_eq!(source, vec![1, 2, 3, 4]);
}

#[test]
fn where1_strings() {
    let source = vec!["red", "green", "blue", "white", "yellow"];
    let result : Vec<&str> = source.iter().filter(|&x| x.contains("w")).map(|&x| x).collect();
    assert_eq!(result, vec!["white", "yellow"]);
}
```

## Second overload

The predicate now takes the index of the item as the first parameter, and the item itself as the
second parameter:

```cs
var nums = new [] { 1, 2, 3, 4 };
var result = nums.Where((idx, x) => idx == 0 || x > 2).ToList();
```

The equivalent Rust functionality is obtained by passing the iterator through the [enumerate]()
method, which returns a tuple `(index, item)`:

```rs
let nums = [1, 2, 3, 4];
let result : Vec<i32> = nums.iter().enumerate().filter(|idx, x| idx == 0 || x > 2).collect();
```

## See also

Rust has a method called
[filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) for the very
common case where you want to filter a collection and then transform each element in some way. It's
slightly more efficient than calling `filter` and passing the result to `map`.

