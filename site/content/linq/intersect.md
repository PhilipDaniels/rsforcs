+++
title = "Intersect"
weight = 99
+++

## Description

[Intersect](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.intersect?view=netframework-4.7.1#System_Linq_Enumerable_Intersect)
is a set-based function. Given a pair of sequences, it returns a new sequence that contains only the
elements that are in both source sequences.

There are 2 overloads:

```cs
public static IEnumerable<TSource> Intersect<TSource>(
    this IEnumerable<TSource> first,
    IEnumerable<TSource> second);

public static IEnumerable<TSource> Intersect<TSource>(
    this IEnumerable<TSource> first,
    IEnumerable<TSource> second,
    IEqualityComparer<TSource> comparer);
```

The first uses a default comparer, the second allows you to specify the comparer.

Usage is simple:

```cs
```

## In Rust, first overload

There is no `intersect` function available on
[Iterator](https://doc.rust-lang.org/core/iter/trait.Iterator.html) and
[itertools](https://docs.rs/crate/itertools/) does not have one either, which is a surprising
omission. The standard library collection type
[HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html) does have an
[intersection](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.intersection)
method so we can get equivalent functionality by explicitly converting our collections to HashSets
first. This is inefficient compared to the [C#
implementation](https://referencesource.microsoft.com/#System.Core/System/Linq/Enumerable.cs,864)
which manages with only a single set and yields its elements lazily.


```rs
use std::collections::HashSet;

#[test]
fn intersect() {
    let first : HashSet<i32> = [10, 20, 30, 10].iter().cloned().collect();
    let second : HashSet<i32> = [10, 60, 20, 70].iter().cloned().collect();
    let mut result : Vec<i32> = first.intersection(&second).cloned().collect();
    result.sort();
    assert_eq!(result, vec![10, 20]);
}
```

The `cloned` calls are to get rid of the references to make the example clearer, and the `sort` call
is just to make the `assert_eq` work.

## In Rust, second overload

If we build the `HashSets` using a custom comparer we can ensure that the call to `intersection`
will produce the data we want. Probably the easiest way is to use Itertools
[unique_by](https://docs.rs/itertools/0.7.7/itertools/trait.Itertools.html#method.unique_by).

```rs

```

## See Also

The other set-based operators, Union, Intersect and Except.

## TODO

Write an `intersect` adapter for itertools and submit a PR.

