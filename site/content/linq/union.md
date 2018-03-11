+++
title = "Union"
weight = 99
+++

## Description

[Union](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.union?view=netframework-4.7.1#System_Linq_Enumerable_Union)
is a set-based function. Given a pair of sequences, it returns a new sequence that contains all the
items in either sequence. All duplicates are removed from the output sequence.

There are 2 overloads:

```cs
public static IEnumerable<TSource> Union<TSource>(
    this IEnumerable<TSource> first,
    IEnumerable<TSource> second);

public static IEnumerable<TSource> Union<TSource>(
    this IEnumerable<TSource> first,
    IEnumerable<TSource> second,
    IEqualityComparer<TSource> comparer);
```

The first uses a default comparer, the second allows you to specify the comparer.

Usage is simple:

```cs
```

## In Rust, first overload

Neither Rust's [Iterator](https://doc.rust-lang.org/core/iter/trait.Iterator.html) trait nor
[itertools](https://docs.rs/crate/itertools/0.7.7) have a built-in Union adapter. However, it is
trivial to implement by using
[chain](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.chain) and then
[unique](./linq/distinct.md) from itertools:

```rs
use itertools::Itertools;

#[test]
fn itertools_union() {
    let first = vec![10, 20, 30, 40, 10];
    let second = vec![10, 20, 50, 40];

    let result : Vec<i32> = first.into_iter().chain(second.into_iter()).unique().collect();
    assert_eq!(result, vec![10, 20, 30, 40, 50]);
}
```

## In Rust, second overload

Instead of calling `unique`, call `unique_by`. There is an example on the [Distinct/Union](./linq/distinct.md) page.

## See Also

The other set-based operators, Union, Intersect and Except.
