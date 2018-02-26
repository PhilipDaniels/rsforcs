+++
title = "Concat"
weight = 99
+++

## Overloads

[Concat](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.concat?view=netframework-4.7.1#System_Linq_Enumerable_Concat)
is used to join two sequences together. It returns all the values from the first sequence, then the
values from the second sequence, then stops. One or both sequences may be empty.

There is only 1 overload:

```cs
public static IEnumerable<TSource> Concat<TSource>(this IEnumerable<TSource> first, IEnumerable<TSource> second);
```

In C# the overload of `Count` which does not take a predicate is optimized when the underlying
collection type is an
[ICollection&lt;T&gt;](https://docs.microsoft.com/en-gb/dotnet/api/system.collections.generic.icollection-1?view=netframework-4.7.1),
because this interface provides a
[Count](https://docs.microsoft.com/en-gb/dotnet/api/system.collections.generic.icollection-1.count?view=netframework-4.7.1)
property which can return the size of the collection in constant time. Since this is the most
commonly used overload, and the most common types of sequences are arrays and lists which both
implement `ICollection<T>`, in practice most calls to `Count` in the C# world are very fast.

```cs
public static int Count<TSource>(this IEnumerable<TSource> source);
public static int Count<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);

public static long LongCount<TSource>(this IEnumerable<TSource> source);
public static long LongCount<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
```

Usage is trivial:

```cs
```


### See Also
