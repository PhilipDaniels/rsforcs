+++
title = "Any and All"
weight = 99
+++

## Description

[Any](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.any?view=netframework-4.7.1#System_Linq_Enumerable_Any)
and
[All](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.all?view=netframework-4.7.1#System_Linq_Enumerable_All)
are used to test sequences. There are 3 overloads:

```cs
public static bool Any<TSource>(this IEnumerable<TSource> source);
public static bool Any<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
public static bool All<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

The version of `Any` without a predicate simply checks whether the sequence is non-empty. In C# it
is faster than `Count() != 0` because it only needs to examine at most one item, rather than
potentially scanning the entire sequence to calculate the count.

The versions which take a predicate and call it against each element in the source sequence. `Any`
will terminate as soon as `true` is returned for an item. `All` will terminate when it reaches the
end of the source sequence or when the predicate first returns `false`.

Usage is very simple:

```cs
```

In Rust, the variants with predicates have direct equivalents in the
[any](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) and
[all](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all) functions:

```rs
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
```

The equivalent of the overload of `Any` without a predicate is a call to [len or count](./linq/count.md).
