+++
title = "Aggregate"
weight = 99
+++

## Description

[Aggregate](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.aggregate?view=netframework-4.7.1#System_Linq_Enumerable_Aggregate)
is the .Net implementation of a very common higher-order function usually called
[fold](https://en.wikipedia.org/wiki/Fold_%28higher-order_function%29), though as the Wikipedia page
mentions, it has many other names.

Whatever name it is going by, it takes a sequence and combines its elements using a function you can
specify, and then returns a single result.

There are 3 overloads:

```cs
public static TSource Aggregate<TSource>(
    this IEnumerable<TSource> source,
    Func<TSource, TSource, TSource> func);

public static TAccumulate Aggregate<TSource, TAccumulate>(
    this IEnumerable<TSource> source,
    TAccumulate seed,
    Func<TAccumulate, TSource, TAccumulate> func);

public static TResult Aggregate<TSource, TAccumulate, TResult>(
    this IEnumerable<TSource> source,
    TAccumulate seed,
    Func<TAccumulate, TSource, TAccumulate> func,
    Func<TAccumulate, TResult> resultSelector);
```

The aggregation starts with a seed of type `TAccumulate` (for the first overload, the seed is the
first element of the sequence and hence `TAccumulate == TSource`).

For each item in the source, we apply the accumulation function `func`. Func takes as its arguments
the current accumulated value, the next item, and returns a new accumulated value.

Once the source sequence has been fully processed, we optionally apply `resultSelector` to the final
accumulator value to produce the value that is returned. `resultSelector` defaults to the identity
function, `x => x`.

In the description above, it was implicit that processing of the source sequence happens in the
usual beginning-to-end order; some languages also have a `foldr` which processes the input sequence
from end-to-beginning instead.

```cs
```

## Rust: the 2nd overload

Rust iterators have a function called
[fold](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.fold). There is no `foldr`,
but you can get the same functionality by first reversing your iterator using
[rev](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.rev).

Rust's fold is the exact equivalent of the second `Aggregate` overload. Let's show that first. The
classic examples is to implement `sum` in terms of `fold`, but since that is the demo in the
official Rust documentation, let's try something a little more elaborate (but still easy):

```rs
#[test]
fn fold2_ints() {
    let source = vec![10, 20, 30, 40];
    let result = source.iter().fold(1000, |acc, x| acc + x * 2);
    assert_eq!(result, 1000 + 20 + 40 + 60 + 80); // 1200
}
```

To calculate `result` we seed the accumulation with 1000, then we add on twice the value of each
element in the source.

You can do string joins this way too:

```rs
#[test]
fn fold2_strings() {
    let source = vec!["alpha", "beta", "gamma"];
    let result = source.iter().fold("".to_string(), |acc, x|
        if acc.len() > 0 { acc + ", " + x } else { x.to_string() });

    assert_eq!(result, "alpha, beta, gamma");
}
```

Though it's probably worth mentioning the [itertools](https://docs.rs/crate/itertools/0.7.7) crate
at this point, as it has an easier to use
[join](https://docs.rs/itertools/0.7.7/itertools/fn.join.html) function.

## Rust: the third overload

The third overload is just a convenience function that allows you to specify a `resultSelector` Func:
if you want this functionality in Rust, just pass the result of `fold` into [map](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.map).

## Rust: the first overload

It's actually the first overload which is a little trickier to duplicate, because we need to get
hold of the first element to act as the seed. Luckily the behaviour of the C# version is that when
it gets the first element it also *consumes* it, so you get the following behaviour:

```cs
var source = new int[] { 10, 20, 30, 40 };
var result = source.Aggregate((acc, x) acc + x);
Assert.AreEqual(result, 100);
```

If `result` was 110 instead (the sum of all the values plus the sum of the first element) we would
have a harder (but not impossible) job. As it is, the Rust equivalent can be formulated like this:

```rs
#[test]
fn fold1_ints() {
    let source = vec![10, 20, 30, 40];
    let mut it = source.iter();
    let seed = *it.nth(0).unwrap();  // Note this will panic if source is empty.
    let result = it.fold(seed, |acc, x| acc + x);
    assert_eq!(result, 100);
}
```

Admittedly, when performing a `sum` over the ints this is rather pointless, because there is a seed
value for which the operation `+` is [idempotent](https://en.wikipedia.org/wiki/Idempotence), namely
a seed of `0`. You might as well just use the `fold` function directly with a seed of 0. If you
wanted a product instead of a sum, you would specify `1` as the seed.
