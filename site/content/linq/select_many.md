+++
title = "SelectMany"
weight = 120
+++

## Description

[SelectMany](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.selectmany?view=netframework-4.7.1#System_Linq_Enumerable_SelectMany)
is used to flatten sequences of sequences. We start with an original sequence with elements of type
`TSource`, then we generate a new sequence whose elements consist of the original `TSource` element
and a subsequence (from `selector` or `collectionSelector` in the overloads below). Then we either
return the subsequence items directly, or we apply another func, `resultSelector`. The `index` is
the index of the item in the first collection.

```cs
public static IEnumerable<TResult> SelectMany<TSource, TResult>(
    this IEnumerable<TSource> source,
    Func<TSource, IEnumerable<TResult>> selector);

public static IEnumerable<TResult> SelectMany<TSource, TResult>(
    this IEnumerable<TSource> source,
    Func<TSource, int, IEnumerable<TResult>> selector);

public static IEnumerable<TResult> SelectMany<TSource, TCollection, TResult>(
    this IEnumerable<TSource> source,
    Func<TSource, IEnumerable<TCollection>> collectionSelector,
    Func<TSource, TCollection, TResult> resultSelector);

public static IEnumerable<TResult> SelectMany<TSource, TCollection, TResult>(
    this IEnumerable<TSource> source,
    Func<TSource, int, IEnumerable<TCollection>> collectionSelector,
    Func<TSource, TCollection, TResult> resultSelector) ;
```

[Jon Skeet](https://codeblog.jonskeet.uk/2010/12/27/reimplementing-linq-to-objects-part-9-selectmany/)
has a very good example which expresses this in terms of files and lines:

```cs
var query = from file in Directory.GetFiles("logs")
            from line in File.ReadLines(file)
            select Path.GetFileName(file) + ": " + line;

// Equivalent extension method syntax:
var query = Directory.GetFiles("logs")
                     .SelectMany(file => File.ReadLines(file),
                                 (file, line) => Path.GetFileName(file) + ": " + line);
```

The original sequence is the `IEnumerable<string>` of filenames returned by `GetFiles`. We then pass
each of those elements into the `File.ReadLines` function, which yields another
`IEnumerable<string>` containing the lines in each file. Finally this is passed to a closure which
takes `(file, line)` for each line in each file. This is an example of overload 3 above
(`Path.GetFileName...` is the `resultSelector`).

Surprisingly, Rust has a direct equivalent called
[flatmap](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.flat_map). Actually it's
not *that* surprising because this is a common method/pattern in functional programming.

Let's write a Rust equivalents of the File/Lines example.

## With a for loop

First let's do it with a for loop, to make it clear which methods we need to call. I'm assuming that
you can get a list of files from somewhere, for now let's hardcode it, and ignore errors by using
`unwrap`:

```rs
let files = vec!["Cargo.toml", ".gitignore"];

for file in &files {
    let f = File::open(file).unwrap();
    let rdr = BufReader::new(f);
    for line in rdr.lines() {
        println!("{}: {}", file, line.unwrap());
    }
}
```

The only significant difference here is the use of a `File` and `BufReader.lines()` to duplicate the
functionality of `File.ReadLines`.

## With iterators

Rewritten to use iterators, it looks like this:

```rs
let files = vec!["Cargo.toml", ".gitignore"];

let result : Vec<String> = files.iter().flat_map(|&file| {
    let f = File::open(file).unwrap();
    let rdr = BufReader::new(f);
    rdr.lines().map(move |line| format!("{}: {}", file, line.unwrap()))
}).collect();
```

Let's break this down.

- First we call `files.iter` which gives us an iterator which yields each filename in turn. This is
  the equivalent of the `IEnumerable<TSource>` collection in C#.
- Then we call `flat_map`. It takes a closure which has each filename as input - `|&file|` , and is
  expected to produce an iterator as its result. `rdr.lines()` gives us an iterator over each line
  in the `file`, and we call `map` on that to do the final formatting. Finally we collect all the
  results into a `Vec<String>`. You don't have to `collect` of course, I just did it in this example
  to make the final type obvious. You could just as easily pass this data onto further iterator
  functions.

On my machine, if I print the result I get this:

```txt
Cargo.toml: [package]
Cargo.toml: name = "selectmany"
Cargo.toml: version = "0.1.0"
Cargo.toml: authors = ["Philip Daniels <Philip.Daniels1971@gmail.com>"]
Cargo.toml:
Cargo.toml: [dependencies]
.gitignore:
.gitignore: /target/
.gitignore: **/*.rs.bk
```

The only part that is slightly tricky is the final `rdr.lines().map` bit; it's the equivalent of
this line in C#:

```cs
(file, line) => Path.GetFileName(file) + ": " + line);
```

Note that we had to `move` into the closure. Without the `move`, we get:

```txt
error[E0597]: `file` does not live long enough
  --> src/main.rs:19:50
   |
19 |         rdr.lines().map(|line| format!("{}: {}", file, line.unwrap()))
   |                         ------                   ^^^^ borrowed value does not live long enough
   |                         |
   |                         capture occurs here
20 |     }).collect();
   |     -          - borrowed value needs to live until here
   |     |
   |     borrowed value only lives until here
```

Basically this error is saying that the `map` closure borrows the `file` value from the surrounding
scope. That borrow only exists to the end of the `map` closure, but it needs to persist until the
`collect` can run. The easy fix is just to move the `file` into the closure.

So it is the `file` which is being moved, not the `line`!

If you get stuck with what the final closure should be returning, one trick is to get the compiler
to help you out. Just return some dumb data:

```rs
let result : Vec<String> = files.iter().flat_map(|&file| {
    let f = File::open(file).unwrap();
    let rdr = BufReader::new(f);
    //rdr.lines().map(move |line| format!("{}: {}", file, line.unwrap()))
    42
}).collect();
```

This generates a wonderful error message which tells us we need to return an iterator, not a scalar
value:

```txt
16 |     let result : Vec<String> = files.iter().flat_map(|&file| {
   |                                             ^^^^^^^^ `{integer}` is not an iterator; maybe try calling `.iter()` or a similar method

```

## The overloads with an index

The two overloads that take an index are, as always, easily simulated by passing the iterator
through the [enumerate](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.enumerate)
adapter. In this case, it is always the first sequence we want to enumerate, which makes it dead
easy:

```rs
let result : Vec<String> = files.iter().enumerate().flat_map(|(file_idx, &file)| {
    let f = File::open(file).unwrap();
    let rdr = BufReader::new(f);
    rdr.lines().map(move |line| format!("File #{}: {}: {}", file_idx, file, line.unwrap()))
}).collect();
```

Which now prints:

```txt
File #0: Cargo.toml: [package]
File #0: Cargo.toml: name = "selectmany"
File #0: Cargo.toml: version = "0.1.0"
File #0: Cargo.toml: authors = ["Philip Daniels <Philip.Daniels1971@gmail.com>"]
File #0: Cargo.toml:
File #0: Cargo.toml: [dependencies]
File #1: .gitignore:
File #1: .gitignore: /target/
File #1: .gitignore: **/*.rs.bk
```

Of course, Rust gives you the flexibility to enumerate both the file and the lines, as in this
example. The C# version can't do this:

```rs
let result : Vec<String> = files.iter().enumerate().flat_map(|(file_idx, &file)| {
    let f = File::open(file).unwrap();
    let rdr = BufReader::new(f);
    rdr.lines().enumerate()
        .map(move |(line_idx, line)| format!("File #{}, Line #{}: {}: {}", file_idx, line_idx, file, line.unwrap()))
}).collect();
```

Which prints

```txt
File #0, Line #0: Cargo.toml: [package]
File #0, Line #1: Cargo.toml: name = "selectmany"
File #0, Line #2: Cargo.toml: version = "0.1.0"
File #0, Line #3: Cargo.toml: authors = ["Philip Daniels <Philip.Daniels1971@gmail.com>"]
File #0, Line #4: Cargo.toml:
File #0, Line #5: Cargo.toml: [dependencies]
File #1, Line #0: .gitignore:
File #1, Line #1: .gitignore: /target/
File #1, Line #2: .gitignore: **/*.rs.bk
```

## Exercise

Simplify this example to write an example of the first two overloads.

## See Also

Closures
