+++
title = "LINQ, Iterables and more"
weight = 100
sort_by = "weight"
+++

## Intro to LINQ

This section contains Rust equivalents of all the LINQ operators. LINQ is fundamentally about 
iterating over sequences of elements and applying functions to those elements, in Rust the 
equivalent concept is the *iterable*, a type which supports iteration via the [Iterator]
(https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait. Many of the `IEnumerable<T>` 
extension methods have direct equivalents in Rust, and those that don't can be simulated easily. 
Of course, Rust also has some iterator methods that LINQ lacks. We will cover all of them in this
section and give equivalencies.

Iteration is commonly expressed using the `for` loop, and there is surprising complexity lurking 
here in Rust, and a gotcha that can trip up the beginner. It's this: the standard looking `for` 
loop in Rust

```rs
for x in collection {
...
}
```

Is **not** directly equivalent to the C# formulation:

```cs
foreach x in collection {
}
``` 

It's to do with the way that the `for` loop de-sugars<sup>1</sup> into Rust code, but basically a
`for` loop in Rust will have move semantics by default. So iterating over a vector will remove 
all the elements from it! 

The following table shows the equivalencies between the semantics, corresponding variable 
expressions, for loops, and iterator methods. Once you memorise these iteration will get a lot 
easier in Rust!


Semantics        | Expression        | for loop                  |Iterator over type `T` 
---------------- | ----------------- | --------------------------|---------------------- 
move             | `let a = b;`      | `for x in collection`     |`.into_iter()`, yields `T` 
read-only borrow | `let a = &b;`     | `for x in &collection`    |`.iter()`, yields `&T` 
mutable borrow   | `let a = &mut b;` | `for x in &mut collection`|`.iter_mut()`, yields `&mut T` 

So to avoid removing the elements from a vector, just iterate over a reference to the vector 
instead using `for x in &collection`. If you want to change the elements as you go, use the `for x
 in &mut collection` form.

I would like to make another important distinction at this point. In C#, if you have a collection
of elements of type `T`, the LINQ methods will be dealing with `Ts`. In Rust, this is only the 
case if you called `into_iter` to get an [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html). 
Since this consumes the original collection, this is not usually what you want. It's more normal to
call `iter` to iterate over the elements via read-only references. So if you have a vector of
integers, for example, in C# your LINQ methods will be passed an `int`, whereas in Rust you will 
get a `&i32` instead. This can be a little freaky at first, but makes perfect sense when you look
at the above table and realise that Rust is giving you the flexibility to iterate in the most
optimal way. That said, it does make some of the Rust equivalents more verbose than their C#
counterparts. Pattern matching can be used to alleviate this to some extent.

## About the samples

Most of the examples in this section use `iter` rather than `into_iter` as I feel this is more usual.

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


Collections and other classes may implement one or more of the Iterator functions, as appropriate.
The freaky thing is that the default for-loop does move semantics rather than borrow. This means
you can consume collections by iterating them! Of course, the compiler will give an error if this is
a problem for your program.

Q: what is `for &x in &collection`? A: The `&x` is a pattern de-referencing the reference that is
returned by the for loop.

BIG TABLE HERE

## See also
For loops
Writing your own custom iterat
or (a sequence of random numbers).
