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
public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, bool> predicate);
public static IEnumerable<TSource> Where<TSource>(this IEnumerable<TSource> source, Func<TSource, int, bool> predicate);
```

## First overload

The Rust equivalent of `Where` is [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter):
For efficiency purposes, `filter` returns an iterator which yields `&T` rather than `T`. This works
well if you just want to refer to the original items, that is, you want a view into the original
collection, perhaps before you pass them into some other iterator for further processing. Rust works
this way for efficiency, if you want a copy, say you need to build a new collection of items which
is separate from the original one, you must clone the elements, and Rust wants you to do that
explicitly.


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
#[test]
fn where1_ints() {
    let source = vec![1, 2, 3, 4];

    // iter() yields &T. filter() takes &T and yields T.
    //
    // So in this very common example, iter() yields items of type &i32, then the closure to
    // filter() takes items of type &&i32 and returns items of type &i32.
    //
    // This is very different to C#, but makes sense when you realize that Rust always attempts
    // to avoid unnecessary copying, and if you want a copy you are made to ask for it. By
    // default you get a reference to the thing in the original collection (imagine if this was
    // a Vec<HugeStructure> rather than a Vec<int>).
    let result1 : Vec<&i32> = source.iter().filter(|&&x| x > 2).collect();
    assert_eq!(result1, vec![&3, &4]);

    // You can get rid of the references by using map().
    let result2 : Vec<i32> = source.iter().filter(|&&x| x > 2).map(|&x| x).collect();
    assert_eq!(result2, vec![3, 4]);

    // Or you can clone each item (assuming it supports the Clone trait). This is fine for
    // small values, but probably not what you want for large values. Note that we put the
    // cloned() call after the filter, so it has fewer items to process.
    // cloned turns an iterator over &T into an iterator over T by cloning each item.
    let result3 : Vec<i32> = source.iter().filter(|&&x| x > 2).cloned().collect();
    assert_eq!(result3, vec![3, 4]);

    // Finally, you can combine the filter and map calls using the filter_map method.
    let result4 : Vec<i32> = source.iter().filter_map(|&x|
        if x > 2 { Some(x) } else { None }
        ).collect();
    assert_eq!(result4, vec![3, 4]);

    // Just to emphasize, the original vector is unchanged.
    assert_eq!(source, vec![1, 2, 3, 4]);
}

#[test]
fn where1_strings() {
    let source = vec!["red", "green", "blue", "white", "yellow"];
    let result : Vec<&str> = source.iter().filter(|&x| x.contains("w")).map(|&x| x).collect();
    assert_eq!(result, vec!["white", "yellow"]);
}
```

So we have several different methods of getting what we want.
[filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) is probably
my favourite, for the simple reason that it makes it obvious what you are going to get. Note this
quote from the online documentation:

Why filter_map and not just filter and map? The key is in this part:

> If the closure returns Some(element), then that element is returned.</span>

In other words, it removes the `Option<T>` layer and just yields `Ts`.

## Second overload

The predicate now takes the index of the item as the first parameter, and the item itself as the
second parameter:

```cs
[Test]
public void Where2_Ints()
{
    var source = new List<int> { 1, 2, 3, 4 };
    var result = source.Where((x, idx) => idx == 0 || x > 2).ToList();
    CollectionAssert.AreEqual(result, new List<int> { 1, 3, 4 });
}
```

The equivalent Rust functionality is obtained by passing the iterator through the
[enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) method, which
returns a tuple `(index, item)`:

```rs
#[test]
fn where2_ints() {
    let source = vec![1, 2, 3, 4];
    let result1 : Vec<i32> = source.iter().enumerate()
        .filter(|&(idx, &x)| idx == 0 || x > 2)
        .map(|(_, &x)| x).collect();

    assert_eq!(result1, vec![1, 3, 4]);
}
```

Let's break that down. `iter` yields `&i32`. Then `enumerate` adds an index and yields a tuple of
type `(usize, &i32)`. These items then get passed to `filter`. Recall that `filter` takes a
reference to the items, so here we have the syntax for a reference to a tuple: `&(idx, &x)`. Then
`filter` yields the items in the way it normally does by stripping off one of the references,
therefore it is yielding items of type `(usize, &i32)`. We give `map` a closure which takes such a
tuple as its argument, then dereference `&x`, finally giving us a `Vec<i32>`. Whew!

This is definitely a case where `filter_map` is clearer:

```rs
#[test]
fn where2_ints_filter_map() {
    let source = vec![1, 2, 3, 4];
    let result1 : Vec<i32> = source.iter().enumerate()
        .filter_map(|(idx, &x)| if idx == 0 || x > 2 { Some(x) } else { None })
        .collect();

    assert_eq!(result1, vec![1, 3, 4]);
}
```

## Second overload with strings

```cs
[Test]
public void Where2_Strings()
{
    var source = new List<string> { "red", "green", "blue", "white", "yellow" };
    var result = source.Where((x, idx) => idx == 0 || x.Contains("w")).ToList();
    CollectionAssert.AreEqual(result, new List<String> { "red", "white", "yellow" });
}
```

And the Rust. Let's use `filter_map` again:

```rs
#[test]
fn where2_strings() {
    let source = vec!["red", "green", "blue", "white", "yellow"];
    let result : Vec<&str> = source.iter().enumerate()
        .filter_map(|(idx, &x)| if idx == 0 || x.contains("w") { Some(x) } else { None })
        .collect();

    assert_eq!(result, vec!["red", "white", "yellow"]);
}
```
## See also

The Clone trait.
