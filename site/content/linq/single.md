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
exceptions, instead we return an `Option`. Similarly to [First and Last](./linq/first_and_last.md),
implementing `Single` and having it return an `Option` gives us a head start on implementing
`SingleOrDefault`.

Let's create an iterator adapter for `Single`. This returns the item if and only if there is one
match, `None` otherwise:

```rs
/// The function `single` does all the work. It could be a free function, but then we would have to
/// use a slightly different syntax:  let a = single(collection.iter());
/// instead of the more normal:       let a = collection.iter().single();
///
/// We have put the function in a trait simply because traits are how we can add methods to an
/// existing type. To all intents and purposes, this is how we write an extension method in Rust.
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

/// What this says is "I want to add the methods in the trait SingleIteratorAdapter to the trait `I`".
/// `I` happens to be any type of Iterator, so we are effectively writing an extension method that
/// works against all types of iterators.
impl<I> SingleIteratorAdapter for I where I : Iterator {
    // Normally an impl block would contains functions here. But in this case, there is only
    // one function and it already has a default definition in the trait, so we don't have to
    // write anything!
}
```

The comments in the above code explain why we have the ceremony of first defining a trait and then
writing an `impl` block for it: this is the Rust equivalent of a C# extension method like this:

```cs
public static T single(this IEnumerable<T> source);
```

C# obviously wins on brevity in this case, but the Rust trait system is a lot more flexible and
powerful than C# extension methods. There is a lot more on this in the traits chapter.

## Using the iterator adapter

We can then call this on existing iterators just like any of the existing iterator methods:

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

This should be obvious by now:

```rs
trait SingleOrDefaultIteratorAdapter : Iterator {
    fn single_or_default(&mut self, default: Self::Item) -> Self::Item {
        match self.next() {
            None => default,
            Some(x) => match self.next() {
                            None => x,
                            Some(_) => default
                        }
        }
    }
}

impl<I> SingleOrDefaultIteratorAdapter for I where I : Iterator { }

#[test]
fn single_or_default_via_iterator_adapter() {
    let source : Vec<i32> = vec![];
    let result = source.iter().single_or_default(&42);
    assert_eq!(&42, result);

    let source = vec![10, 20, 30, 40];
    let result = source.iter().single_or_default(&42);
    assert_eq!(&42, result);

    let source = vec![10];
    let result = source.iter().single_or_default(&42);
    assert_eq!(&10, result);
}
```

## See Also

[First and Last](./linq/first_and_last.md) are very similar.
