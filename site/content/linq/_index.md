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

Sematics         | Expression        | Iterator       | for loop
--------         | ----------        | --------       | --------
move             | `let a = b;`      | `.into_iter()` | `for x in collection`
read-only borrow | `let a = &b;`     | `.iter()`      | `for x in &collection`
mutable borrow   | `let a = &mut b;` | `.iter_mut()`  | `for x in &mut collection`

Collections and other classes may implement one or more of the Iterator functions, as appropriate.
The freaky thing is that the default for-loop does move semantics rather than borrow. This means
you can consume collections by iterating them! Of course, the compiler will give an error if this is
a problem for your program.

Q: what is `for &x in &collection`? A: The `&x` is a pattern de-referencing the reference that is
returned by the for loop.

BIG TABLE HERE

## See also
For loops
Writing your own custom iterator (a sequence of random numbers).
