+++
title = "Where"
weight = 100
sort_by = "weight"
+++

## Overloads

[Where](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.where?view=netframework-4.7.1#System_Linq_Enumerable_Where)
is used to filter a sequence based on a predicate. There are two overloads of the `Where` method,
the first one passes each item to the predicate, the second overload passes the item and the index
of the item:

```cs
1: public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
2: public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, int, bool> predicate);
```

## First overload

The Rust equivalent of `Where` is [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter):

```cs
[Test]
public void Where1_Ints()
{
    var source = new List<int> { 1, 2, 3, 4 };
    var result = source.Where(x => x > 2).ToList();
    CollectionAssert.AreEqual(result, new List<int> { 3, 4 });
}

[Test]
public void Where1_Strings()
{
    var source = new List<string> { "red", "green", "blue", "white", "yellow" };
    var result = source.Where(x => x.Contains("w")).ToList();
    CollectionAssert.AreEqual(result, new List<String> { "white", "yellow" });
}
```

```rs
let nums = [1, 2, 3, 4];
let result : Vec<i32> = nums.iter().filter(|x| x > 2).collect();
```

## Second overload

The predicate now takes the index of the item as the first parameter, and the item itself as the
second parameter:

```cs
var nums = new [] { 1, 2, 3, 4 };
var result = nums.Where((idx, x) => idx == 0 || x > 2).ToList();
```

The equivalent Rust functionality is obtained by passing the iterator through the [enumerate]()
method, which returns a tuple `(index, item)`:

```rs
let nums = [1, 2, 3, 4];
let result : Vec<i32> = nums.iter().enumerate().filter(|idx, x| idx == 0 || x > 2).collect();
```

## See also

Rust has a method called
[filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) for the very
common case where you want to filter a collection and then transform each element in some way. It's
slightly more efficient than calling `filter` and passing the result to `map`.

