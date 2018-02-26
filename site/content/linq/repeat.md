+++
title = "Repeat"
weight = 97
+++

## Description

[Repeat](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.repeat?view=netframework-4.7.1#System_Linq_Enumerable_Repeat)
is used to yield a specific item N times. There is only 1 overload, and it is a simple static method
rather than an extension method:

```cs
public static IEnumerable<TResult> Repeat<TResult>(TResult element, int count);
```

Usage is trivial:

```cs
```

Rust also has a [repeat](https://doc.rust-lang.org/std/iter/fn.repeat.html) function, but it doesn't
take a count and repeats forever. We get the equivalent functionality by combining with
[take](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take):

```rs
use std::iter;

#[test]
fn repeat_ints1() {
    let mut source = iter::repeat(4).take(2);
    assert_eq!(Some(4), source.next());
    assert_eq!(Some(4), source.next());
    assert_eq!(None, source.next());
}

#[test]
fn repeat_ints2() {
    let source = iter::repeat(4).take(2);
    let result : Vec<i32> = source.collect();
    assert_eq!(result, vec![4, 4]);
}

#[test]
fn repeat_string_slices() {
    let source = iter::repeat("hello").take(2);
    let result : Vec<&str> = source.collect();
    assert_eq!(result, vec!["hello", "hello"]);
}
```

## See Also

The `cycle` function, which repeats an entire sequence endlessly, rather than just one item.
