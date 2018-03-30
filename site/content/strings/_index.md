+++
title = "Strings"
weight = 40
sort_by = "weight"
+++

## Rust Strings

As in most modern languages, Rust has first-class support for strings. The main difference between Rust and C# is that
in Rust there are two main string types [str](https://doc.rust-lang.org/std/primitive.str.html) and 
[String](https://doc.rust-lang.org/std/string/struct.String.html). `str` is a *string slice* and represents a
fixed-length sequence of characters. It is usually seen in its borrowed form `&str`. `String` is the owned equivalent
of `str`, it consists of a heap-allocated `Vec<u8>` which can grow or shrink in size.

Strings and their corresponding slices are laid out in memory like this:

 

#### Major Points

* In C# strings are immutable. Changing a string in any way creates a new string, with a corresponding heap allocation.
* In Rust, strings are mutable. 
* There is no [StringBuilder](https://msdn.microsoft.com/en-us/library/system.text.stringbuilder(v=vs.110).aspx) in Rust.
  `StringBuilder` is needed in C# to allow strings to be built-up efficiently from many smaller pieces. It basically
  optimizes allocations using an internal buffer. In Rust, `Strings` are backed by a `Vec<u8>` which is
  *already efficient* at managing allocations. Where you would reach for a `StringBuilder` in C#, in Rust you can
  just call the required methods on the `String`.
* C# has no equivalent of the `str` type, so dealing with substrings is inefficient - you usually end up creating
  a new string (...another allocation...). Rust programmers like to avoid allocations, and make extensive use of `&str`
  as a sort of 'view' into a `String`. Since strings are so ubiquitous, this is a significant performance win for Rust.
  C# will soon be getting [Span<T>](https://msdn.microsoft.com/en-us/magazine/mt814808.aspx) for this very reason.    
* C# strings are stored as a sequence of [Char](https://docs.microsoft.com/en-us/dotnet/api/system.char?view=netframework-4.7.1),
  which are [UTF-16](https://en.wikipedia.org/wiki/UTF-16). Rust strings are stored as [UTF-8](https://en.wikipedia.org/wiki/UTF-8).
  Both encodings are variable length, but if the text is ASCII-heavy then UTF-8 will be more compact.
* A Rust `String` is always valid `UTF-8`. All the methods that mutate strings guarantee to preserve this invariant. It
  is not true that a C# string is always valid `UTF-16`. See section 'Char objects and Unicode characters' under the
  [Remarks](https://msdn.microsoft.com/en-us/library/system.string(v=vs.110).aspx#Remarks). 
* A Rust `str` is also always valid `UTF-8`. The `Range` used for indexing *does* take a byte offset into the
  underlying `Vec<u8>`, but if you try to index 'half-way' into a character it will panic with a nice message like:
  `panicked at 'byte index 2 is not a char boundary; it is inside 'თ' (bytes 0..3) of 'თhello world'`.
* Amazingly, given that `UTF-16` is a variable length encoding, C# strings can be mutably indexed using `str[n]`.
  Because of the variable length, you will get back a `Char` but you might not be dealing with an entire Unicode
  code point. Rust strings cannot be indexed, to avoid this problem. If you need to deal with the string 'by chars',
  there is an iterator called [chars](https://doc.rust-lang.org/std/string/struct.String.html#method.chars) to do it.
* C# strings have a bunch of [Format](https://msdn.microsoft.com/en-us/library/dn906224(v=vs.110).aspx) methods.
  In Rust, formatting is achieved with the [format!](https://doc.rust-lang.org/std/fmt/) macro.
* In C# the `String` type is ubiquitous and is used to also represent 'external' items like filenames passed to
  [File.Open()](https://msdn.microsoft.com/en-us/library/b9skfh7s(v=vs.110).aspx) and command line arguments received
  from the operating system in the [main method](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/main-and-command-args/).
  This is a problem because those externalities may use representations that are not valid C# strings.
  Rust, which chose to guarantee that a `String` is *always* valid `UTF-8`, had to find a different way to deal with this.
  The language designers chose to use separate types for these cases, namely
  [Path](https://doc.rust-lang.org/std/path/struct.Path.html)/[PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
  for interacting with the filesystem, and
  [OsStr](https://doc.rust-lang.org/std/ffi/struct.OsStr.html)/[OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html).
  The duality is because there are slice/owned versions of each, similar to the str/String dichotomy.
  

  

- String methods
- formatting
- parsing
- joining (csv)
- other string types (Path, PathBuf)
