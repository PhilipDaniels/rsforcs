+++
title = "Select"
weight = 90
+++

## Overloads

[Select](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.select?view=netframework-4.7.1#System_Linq_Enumerable_select)
is used to project each member of a sequence into a new form. There are two overloads of the `Select` method,
the first one passes each item to the predicate, the second overload passes the item and the index
of the item:

```cs
public static IEnumerable<TResult> Select<TSource, TResult>(this IEnumerable<TSource> source, Func<TSource, TResult> selector);
public static IEnumerable<TResult> Select<TSource, TResult>(this IEnumerable<TSource> source, Func<TSource, int, TResult> selector);
```

## First overload

The Rust equivalent of `Select` is
[map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map). Arguably, it's a better
name, since transforming a sequence of A into a sequence of B is commonly known as
[mapping](https://en.wikipedia.org/wiki/Map_(higher-order_function)).

Note that in both languages, the types of A and B can be different or the same.

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
fn select1_ints() {
    let source = vec![10, 20, 30, 40];
    let result : Vec<String> = source.iter().map(|x| format!("Hello {}", x)).collect();
    assert_eq!(result, vec!["Hello 10", "Hello 20", "Hello 30", "Hello 40"]);
}

#[test]
fn select1_strings() {
    let source = vec!["red", "green", "blue", "white", "yellow"];
    let result : Vec<usize> = source.iter().map(|x| x.len()).collect();
    assert_eq!(result, vec![3, 5, 4, 5, 6]);
}
```

## Second overload

The equivalent Rust functionality is obtained by passing the iterator through the
[enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) method, which
returns a tuple `(index, item)`.

Note that both languages enumerate from 0.

```cs
[Test]
public void Select2_Ints()
{
    var source = new List<int> { 10, 20, 30, 40 };
    var result = source.Select((x, idx) => "Hello " + (idx * x)).ToList();
    CollectionAssert.AreEqual(result, new List<String> { "Hello 0", "Hello 20", "Hello 60", "Hello 120" });
}

public void Select2_Strings()
{
    var source = new List<int> { "red", "green", "blue", "white", "yellow" };
    var result = source.Select((x, idx) => "Hello " + (idx * x)).ToList();
    CollectionAssert.AreEqual(result, new List<String> { "Hello 0", "Hello 20", "Hello 60", "Hello 120" });
}
```

```rs
#[test]
fn select2_ints() {
    let source = vec![10, 20, 30, 40];
    let result : Vec<String> = source.iter().enumerate()
        .map(|(idx, x)| format!("Hello {}", idx * x))
        .collect();

    assert_eq!(result, vec!["Hello 0", "Hello 20", "Hello 60", "Hello 120"]);
}

#[test]
fn select2_strings() {
    let source = vec!["red", "green", "blue", "white", "yellow"];

    // Let's make a tuple this time.
    let result : Vec<(String, usize)> = source.iter().enumerate()
        .map(|(idx, x)| (format!("Hello {}", x), idx))
        .collect();

    assert_eq!(result, vec![
        ("Hello red".to_string(),    0),
        ("Hello green".to_string(),  1),
        ("Hello blue".to_string(),   2),
        ("Hello white".to_string(),  3),
        ("Hello yellow".to_string(), 4)
    ]);
}
```

In the first example, `result` is a `Vec<String>`, and the `assert_eq` call works because there is
an automatic conversion between `String` and `&str`. In the second example, the `String` is buried
inside the tuple and there is no automatic conversion, so we have to call `to_string` to ensure that
the types match exactly.
