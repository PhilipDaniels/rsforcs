+++
title = "DefaultIfEmpty"
weight = 96
+++

## Description

[DefaultIfEmpty](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.defaultifempty?view=netframework-4.7.1#System_Linq_Enumerable_DefaultIfEmpty)
is one of the more rarely used LINQ functions. Its behaviour is simple: if given a non-empty
sequence it yields that sequence. If given an empty sequence then it yields a sequence with a single
element in it, the 'default'. There are two overloads which differ in how the default value is
constructed, the first uses the `default(T)` syntax - so for reference types it's `null`, for ints
it's `0` etc. The second overload allows you to specify your own default value:

```cs
public static IEnumerable<TSource> DefaultIfEmpty<TSource>(this IEnumerable<TSource> source);
public static IEnumerable<TSource> DefaultIfEmpty<TSource>(this IEnumerable<TSource> source, TSource defaultValue);
```

Usage is trivial:

```cs
```

There is no direct equivalent in Rust. The easiest way to get the equivalent functionality is to
write an iterator adapter (See [single](./linq/single.md) for an in-depth explanation of this code).

TODO: Write me.
