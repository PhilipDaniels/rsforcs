+++
title = "Distinct"
weight = 99
+++

## Description

[Distinct](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.distinct?view=netframework-4.7.1#System_Linq_Enumerable_Distinct)
is a set-based function. Given a sequence, it returns a new sequence that contains only distinct (or
unique) items.

There are 2 overloads:

```cs
public static IEnumerable<TSource> Distinct<TSource>(
    this IEnumerable<TSource> source);

public static IEnumerable<TSource> Distinct<TSource>(
    this IEnumerable<TSource> source,
    IEqualityComparer<TSource> comparer);
```

The first uses a default comparer, the second allows you to specify the comparer.

Usage is simple:

```cs
```

## In Rust, first overload

Rust's [Iterator](https://doc.rust-lang.org/core/iter/trait.Iterator.html) trait does not have a
built-in way of unique-ing items, but it is easy to do a crude version by simply collecting all the
elements of the source into a HashSet and then iterating that:

```rs
use std::collections::HashSet;

#[test]
fn built_in_distinct() {
    let source = vec![10, 20, 10, 40, 20, 30, 10, 40];
    let result : HashSet<i32> = source.into_iter().collect();
    assert_eq!(result.len(), 4);
    assert!(result.contains(&10));
    assert!(result.contains(&20));
    assert!(result.contains(&30));
    assert!(result.contains(&40));

    // This prints 20, 10, 40, 30 on my machine.
    // Ideally we would like 10, 20, 40, 30.
    for v in &result {
        println!("v = {}", v);
    }
}
```

However, this differs from the C# equivalent in two ways (which may or may not be important).
Firstly, it does not yield the elements lazily, and secondly the elements are not guaranteed to come
out of the `HashSet` in the same order as they were seen in the original source sequence.

The [itertools](https://docs.rs/crate/itertools/0.7.7) crate comes to our rescue here, it has a
[unique](https://docs.rs/itertools/0.7.7/itertools/trait.Itertools.html#method.unique) function
ready to go which behaves just like the C# one:

```rs
use itertools::Itertools;

#[test]
fn itertools_unique() {
    let source = vec![10, 20, 10, 40, 20, 30, 10, 40];
    let result : Vec<i32> = source.into_iter().unique().collect();
    assert_eq!(result, vec![10, 20, 40, 30]);
}
```

Behind the scenes both the [C#
implementation](https://github.com/Microsoft/referencesource/blob/master/System.Core/System/Linq/Enumerable.cs#L826)
and the [itertools
implemention](https://github.com/bluss/rust-itertools/blob/master/src/unique_impl.rs) use a
HashSet internally.

## In Rust, second overload

The second C# overload takes a custom comparer: a class that places two values into an ordering
relationship (less than, equal, greater than). Now is not the time to get into the complexities of
what it means for something to be equal in .Net, safe to say this overload is typically used when
you want to transform the values in the sequence in some way before testing them for equality. You
might want to extract a sub-value, for example you might want to compare `Car` objects by
`Car.EngineSize`, or you might want to transform the entire value. Comparing strings
case-insensitively is a classic example. Itertools has
[unique_by](https://docs.rs/itertools/0.7.7/itertools/trait.Itertools.html#method.unique_by) for
just this scenario:

```rs
#[test]
fn itertools_unique_by() {
    let source = vec!["hello", "world", "HELLO", "Rust", "rust"];
    let result : Vec<&'static str> = source.into_iter().unique_by(|s| s.to_lowercase()).collect();
    assert_eq!(result, vec!["hello", "world", "Rust"]);
}
```
