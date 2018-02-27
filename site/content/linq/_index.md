+++
title = "LINQ, Iterables and more"
weight = 100
sort_by = "weight"
+++

## Intro to LINQ

All Linq examples should be self contained. I will often call `ToList()` or `collect()` to ensure
the enumerable is evaluated, to aid in debugging and display of the type.

In Rust, we usually get an iterator by calling the `iter` method.

There are usually two sets of examples: one against ints, and one against strings. The first
is an exemplar of a value type in C#, and the second a reference type.

I tend to use vecs rather than arrays or slices, but the examples all equally to all those types.

The C# examples are written as unit tests using Framework 4.7.1 and the
[NUnit](https://github.com/nunit/docs/wiki/NUnit-Documentation) testing framework. The full set of
examples is available as a VS2017 solution in this website's [Github
repo](https://github.com/PhilipDaniels/rsforcs).

Ensure we cover MoreLinq and all the Rust iter methods, and probably itertools too.

The Empty page has a nice description of why there are custom structs rather than `IEnumerable<T>`
returned by everything.

TODO: Writing your own iterators. Sequence of random numbers.

BIG TABLE HERE

## See also

Writing your own custom iterator
iter, iter_mut, into_iter
