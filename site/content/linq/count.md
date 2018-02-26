+++
title = "Count and LongCount"
weight = 98
+++

## Description

[Count](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.count?view=netframework-4.7.1#System_Linq_Enumerable_Count)
and
[LongCount](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.longcount?view=netframework-4.7.1#System_Linq_Enumerable_LongCount)
are used to count the items in a sequence (surprise!). `LongCount` is necessary for very long
sequences - more than 2<sup>32</sup>-1 items - but is rarely used.

There are 2 overloads of each function. The second overload takes a predicate, and returns a count
of the number of items for which the predicate returns true.

In C# the overload of `Count` which does not take a predicate is optimized when the underlying
collection type is an
[ICollection&lt;T&gt;](https://docs.microsoft.com/en-gb/dotnet/api/system.collections.generic.icollection-1?view=netframework-4.7.1),
because this interface provides a
[Count](https://docs.microsoft.com/en-gb/dotnet/api/system.collections.generic.icollection-1.count?view=netframework-4.7.1)
property which can return the size of the collection in constant time. Since this is the most
commonly used overload, and the most common types of sequences are arrays and lists which both
implement `ICollection<T>`, in practice most calls to `Count` in the C# world are very fast.

```cs
public static int Count<TSource>(this IEnumerable<TSource> source);
public static int Count<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);

public static long LongCount<TSource>(this IEnumerable<TSource> source);
public static long LongCount<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

Usage is trivial:

```cs
```

### Count without a predicate

In Rust, we use the
[len](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html#method.len) or the
[count](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) functions. Why the
difference? `len` is a method of the trait
[ExactSizeIterator](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html), which is the
type of iterator you get when you call `iter` on a sequence which knows exactly how many elements it
has. A lot of things return `ExactSizeIterator` - vectors, arrays, slices, hash maps, among others.
See the
[documentation](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html#implementors).

The two functions have different behaviour though - `len` does not consume the sequence (it doesn't
have to, the sequence always knows its length), but `count` does. This means that you can call `len`
multiple times, but trying to call `count` twice on the same iterator will generate a "used after
move" error.

> Summary: if a call to `len` compiles, use it! Otherwise you will have to fall back on `count`.

Non-consuming, multiple calls work fine:

```rs
#[test]
fn len_ints() {
    let source = vec![10, 20, 30, 40];
    let i = source.iter();
    assert_eq!(4, i.len());
    assert_eq!(4, i.len());
}
```

Whereas this won't compile:

```rs
#[test]
fn count_ints() {
    let source = vec![10, 20, 30, 40];
    let i = source.iter();
    assert_eq!(4, i.count());
    assert_eq!(4, i.count());
}
```

```txt
error[E0382]: use of moved value: `i`
  --> src/linq/count_tests.rs:18:23
   |
17 |         assert_eq!(4, i.count());
   |                       - value moved here
18 |         assert_eq!(4, i.count());
   |                       ^ value used here after move
   |
   = note: move occurs because `i` has type `std::slice::Iter<'_, i32>`, which does not implement the `Copy` trait

error: aborting due to previous error
```

## Count with a predicate

Rust does not support function overloading so there is no version of `len` or `count` which takes a
predicate. We get the equivalent functionality by inserting a call to [filter](./linq/where.md). Since `filter`
cannot possibly know ahead of time how many elements will pass the predicate it cannot return an
`ExactSizeIterator`, therefore you can't call `len`, and must use `count`. Such calls will therefore
be an O(n) operation rather than an O(1) operation. The C# methods suffer from the same problem.

```rs
#[test]
fn len_ints_with_predicate() {
    let source = vec![10, 20, 30, 40];
    let i = source.iter().filter(|&&x| x > 20);
    assert_eq!(2, i.count());
}
```

This is the compilation error you will get if you try to call `len`:

```txt
error[E0599]: no method named `len` found for type `std::iter::Filter<std::slice::Iter<'_, {integer}>, [closure@src/linq/count_tests.rs:25:38: 25:50]>` in the current scope
  --> src/linq/count_tests.rs:26:25
   |
26 |         assert_eq!(2, i.len());
   |                         ^^^

```

## Count vs LongCount

Lastly, are the Rust variants `int` returning or `long` returning? Both `len` and `count` return
`usize`, which means the size will be `u32` on 32-bit platforms, and `u64` on 64-bit platforms.
Since most modern computers run a 64-bit OS, the Rust functions are equivalent to the "Long"
variants.
