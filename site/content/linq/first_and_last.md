+++
title = "First and Last"
weight = 99
+++

## Description

[First](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.first?view=netframework-4.7.1#System_Linq_Enumerable_First)
and
[Last](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.last?view=netframework-4.7.1#System_Linq_Enumerable_Last)
are used to extract the first or last item of a sequence, respectively. They both throw an exception
if they are used on an empty sequence:

```cs
public static TSource First<TSource>(this IEnumerable<TSource> source);
public static TSource First<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
public static TSource Last<TSource>(this IEnumerable<TSource> source);
public static TSource Last<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

In addition there is a second set of overloads named
[FirstOrDefault](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.firstordefault?view=netframework-4.7.1#System_Linq_Enumerable_FirstOrDefault)
and
[LastOrDefault](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.lastordefault?view=netframework-4.7.1#System_Linq_Enumerable_LastOrDefault).
Instead of throwing an exception when used on an empty sequence, these overloads return
`default(TSource)`. For reference types this is always `null`, for value types it is the
'zero-initialized' value.

```cs
public static TSource FirstOrDefault<TSource>(this IEnumerable<TSource> source);
public static TSource FirstOrDefault<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
public static TSource LastOrDefault<TSource>(this IEnumerable<TSource> source);
public static TSource LastOrDefault<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

As usual, there is no direct Rust equivalent of the overloads that take a predicate. Instead, the
equivalent result can be obtained by passing the source sequence through a call to [filter](./linq/where.md) first,
so we will not discuss them further.

## Rust equivalent of First

The direct equivalent is the iterator workhorse function,
[next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next). But note that Rust
does not have exceptions, instead we return an `Option`.

```rs
#[test]
fn first_ints() {
    // On an empty sequence, we get back a None.
    let source : Vec<i32> = vec![];
    assert_eq!(None, source.iter().next());

    // On a non-empty sequence, we get back a Option<&T> which we have to unwrap and then
    // de-reference using '*'. Admittedly, this is a bit verbose.
    let source = vec![10, 20, 30, 40];
    assert_eq!(10, *source.iter().next().unwrap());

    // The fact that we get back an Option leads us to an easy method of implementing the
    // `OrDefault` overloads, using the `unwrap_or*` functions on an Option. The most direct
    // equivalent uses `unwrap_or_default`, but this works only if the element type implements
    // the Default trait, and it probably won't do because by using `iter` you are getting
    // references to things rather than things. You can use `unwrap_or`, or `unwrap_or_else` to
    // calculate the default value using a closure.
    //
    // This is rather ugly, but at least we can specify our own default.
    // Dereference as above.
    let source : Vec<i32> = vec![];
    assert_eq!(&42, source.iter().next().unwrap_or(&42));
}
```

As the comments note, this is not as elegant as the C# version due to `iter` producing `&T` rather
than `T` elements.

## Rust equivalent of Last

Simply replace the calls to `next` above with calls to
[last](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.last) instead.

## Reducing verbosity by writing an iterator adapter

TODO Can we do this better? It would be nice to automatically deref the thing if it is a reference.

```rs
trait FirstIteratorAdapter : Iterator {
    fn first_or_default(&mut self, default: Self::Item) -> Self::Item {
        self.next().unwrap_or(default)
    }
}

impl<I> FirstIteratorAdapter for I where I : Iterator { }

#[test]
fn first_ints_using_iterator_adapter() {
    // On an empty sequence, we get back a None.
    let source : Vec<i32> = vec![];
    assert_eq!(&42, source.iter().first_or_default(&42));

    // On a non-empty sequence, we get back a Option<&T> which we have to unwrap and then
    // de-reference using '*'. Admittedly, this is a bit verbose.
    let source = vec![10, 20, 30, 40];
    assert_eq!(10, *source.iter().first_or_default(&42));
}
```

## See Also

```txt
[single](./linq/single.md) is a little trickier and has its own page.
```