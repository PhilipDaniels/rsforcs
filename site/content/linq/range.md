+++
title = "Range"
weight = 95
+++

## Overloads

[Range](https://docs.microsoft.com/en-gb/dotnet/api/system.linq.enumerable.range?view=netframework-4.7.1#System_Linq_Enumerable_Range)
is used to generate a sequence of integer numbers, starting at a particular number and continuing
for a specified count. There is only 1 overload, and it is a simple static method rather than an
extension method:

```cs
public static IEnumerable<int> Range(int start, int count);
```

Usage is trivial:

```cs
```

The equivalent in Rust is provided most succintly by the range operator `..`. This is typically used
with for loops:

```rs
for i in 0..10 {
    println!("i = {}", i);  // prints 0 to 9
}
```

Note that the range operator generates a *half-open range*, which includes the lower bound but
excludes the upper bound.

Use of the range operator is not confined to for loops. You can use it anywhere that you need a
sequence of integers.

```rs
fn main() {
    let nums : Vec<i32> = (20..25).collect();
    println!("{:?}", nums);  // prints 20 to 24
}
```

Behind the scenes, the compiler uses a [Range](https://doc.rust-lang.org/std/ops/struct.Range.html)
struct to represent the sequence.

## See Also

Ranges are used extensively to generate [slices]().
