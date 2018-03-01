+++
title = "Single"
weight = 99
+++

## Description

[Single](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.single?view=netframework-4.7.1#System_Linq_Enumerable_Single)
and
[SingleOrDefault](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.singleordefault?view=netframework-4.7.1#System_Linq_Enumerable_SingleOrDefault)
are used to extract a single item from a sequence. The difference between `Single` and [first and
last](./linq/first_and_last.md) is that `Single` throws an exception if there is more than one
matching element.

There are 4 functions:

```cs
public static TSource Single<TSource>(this IEnumerable<TSource> source);
public static TSource Single<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
public static TSource SingleOrDefault<TSource>(this IEnumerable<TSource> source);
public static TSource SingleOrDefault<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

As usual, there is no direct Rust equivalent of the overloads that take a predicate. Instead, the
equivalent result can be obtained by passing the source sequence through a call to
[filter](./linq/where.md) first, so we will not discuss them further.

## Rust equivalent of Single

There is no direct equivalent, and the verbosity of trying to compose existing iterator methods to
duplicate the functionality is such that we need to write our own. But note that Rust does not have
exceptions, instead we return an Option. Similarly to [First and Last](./linq/first_and_last.md),
implementing `Single` and having it return an `Option` gives us a head start on implementing
`SingleOrDefault`.

Let's create an iterator adapter for `Single`. This returns the item if and only if there is one
match, `None` otherwise:

```rs
trait SingleIteratorAdapter : Iterator {
    fn single(&mut self) -> Option<Self::Item>
{
        match self.next() {
            None => None,
            Some(x) => match self.next() {
                None => Some(x),
                Some(_) => None
            }
        }
    }
}

impl<I> SingleIteratorAdapter for I where I : Iterator { }
```

We can then call this on existing iterators:

```rs
#[test]
fn single_ints_via_iterator_adapter() {
    let source = vec![10];
    let result = source.iter().single().unwrap();
    assert_eq!(&10, result);

    let source : Vec<i32> = vec![];
    let result = source.iter().single();
    assert_eq!(None, result);

    let source = vec![10, 20];
    let result = source.iter().single();
    assert_eq!(None, result);
}
```

## Rust equivalent of SingleOrDefault

## See Also

[First and Last](./linq/first_and_last.md) are very similar.
