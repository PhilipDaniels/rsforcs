+++
title = "Empty"
weight = 96
+++

## Description

[Empty](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.empty?view=netframework-4.7.1#System_Linq_Enumerable_Empty)
is used to generate an empty sequence of the appropriate type. There is only 1 overload, and it is a
simple static method rather than an extension method:

```cs
public static IEnumerable<TResult> Empty();
```

Usage is trivial:

```cs
```

The equivalent in Rust is the [empty](https://doc.rust-lang.org/std/iter/fn.empty.html) function from [std::iter](https://doc.rust-lang.org/std/iter/index.html):

```rs
use std::iter;

#[test]
fn empty() {
    let mut empty_iter = iter::empty::<i32>();
    assert_eq!(None, empty_iter.next());
}
```

Note that because `empty` has no parameters Rust is not able to infer its type, so we have to help
it out by specifying `::<i32>`. Alternatively, you can specify the type in the `let` statement:

```rs
use std::iter::{self, Empty};

#[test]
fn empty2() {
    let mut empty_iter : Empty<i32> = iter::empty();
    assert_eq!(None, empty_iter.next());
}
```

But you must do one or the other.

This example shows one of the interesting differences between LINQ and Rust's iterators: in C#
pretty much everything is an `IEnumerable<T>`, whereas in Rust each iterator method tends to return
its own custom struct type which contains all the state needed to implement that specific iterator
type. In the case above, the custom structure is the `Empty<i32>`. The `iter` module contains a
[summary](https://doc.rust-lang.org/std/iter/index.html#structs) of all those structure types. This
difference can make the Rust iterator function signatures much harder to read than the equivalent
LINQ ones at first sight, but if you start by assuming that the functions all return something which
can yield items in a sequence and don't worry too much about the specific structs returned you will
be half way there. Furthermore, the type of the item yielded will usually be the same that you
passed in, unless you use one of the `map` variants to return a different type.
