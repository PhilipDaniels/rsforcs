+++
title = "Concat"
weight = 99
+++

## Description

[Concat](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.concat?view=netframework-4.7.1#System_Linq_Enumerable_Concat)
is used to join two sequences together. It returns all the values from the first sequence, then the
values from the second sequence, then stops. One or both sequences may be empty.

There is only 1 overload:

```cs
public static IEnumerable<TSource> Concat<TSource>(this IEnumerable<TSource> first, IEnumerable<TSource> second);
```

Usage is very simple:

```cs

```

In Rust, the equivalent is the
[chain](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain) method.
The behaviour is exactly the same:

```rs
 #[test]
fn chain_ints() {
    // Using into_iter() to avoid having to call filter_map to
    // get a Vec<i32> instead of a Vec<&i32>.
    let v1 = vec![10, 20].into_iter();
    let v2 = vec![30, 40].into_iter();
    let result : Vec<i32> = v1.chain(v2).collect();
    assert_eq!(result, vec![10, 20, 30, 40]);
}
```
