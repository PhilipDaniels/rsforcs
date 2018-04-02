+++
title = "Commonly Used Traits"
weight = 40
sort_by = "weight"
+++

A *trait* is something like an interface - a grouping of functions that together serve a circumscribed purpose. In Rust,
there is a small set of traits which are very commonly used - you are almost sure to use at least one of them in any
program you write, so being familiar with them is a necessity. Luckily, most of them are very simple, consisting of
only one function!

Here is a summary. Traits that can be auto-derived are marked with a `*`.

<p>&nbsp;</p>

## Traits relating to the lifecycle of a value

Trait   | Description | Usage |
------- | ----------- | ----- |
[Default*](https://doc.rust-lang.org/std/default/trait.Default.html) | For creating default values | To simplify creation of large structs; when a default is needed in generic code |
[Clone*](https://doc.rust-lang.org/std/clone/trait.Clone.html) | For making independent copies of a value | 'Independent' usually implies making new heap allocations so this can be expensive | 
[Copy*](https://doc.rust-lang.org/std/marker/trait.Copy.html) | Marker trait for types that can be bit-copied | An extension of `Clone`, only for types that do not own resources |
[Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) | Destructor. | For when you need custom clean-up code |

Some rules:

* If you implement `Drop`, then you can't be `Copy`
* If any field in your struct is not `Copy`, then the struct can't be `Copy` either
* If you implement `Copy` then implement `Clone` as well

<p>&nbsp;</p>

## Traits for converting values

Trait   | Description | Usage |
------- | ----------- | ----- |
[Into](https://doc.rust-lang.org/std/convert/trait.Into.html) | For type X, take ownership of X-value and convert to type T | Making function parameters more flexible (pseudo-overloading) |
[From](https://doc.rust-lang.org/std/convert/trait.From.html) | For type X, convert to type T | Generic way of writing single-argument constructors |
[TryInto](https://doc.rust-lang.org/std/convert/trait.TryInto.html) | Version of `Into` to use if the conversion can fail | Nightly-only |
[TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) | Version of `From` to use if the conversion can fail | Nightly-only |
[ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) | Make an owned value from a reference | When `Clone` is not flexible enough |
[FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) | Make a value by parsing a `&str` | A more specialized version of `From` that can return an error |
[IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) | For type X, specify how it converts to an [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) | Generic bounds: accept any type of iterator | 
[FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) | Build a value from the items in an iterator | Make it easy to construct specialist collection types |

n.b. If you implement `From` there is no need to implement `Into`, the standard library will do it for you automatically
with this implementation:

```rs
impl<T, U> Into<U> for T where U: From<T>
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

`AsRef` (see below) is also used to make function parameters more flexible.

<p>&nbsp;</p>

## Traits for converting references

Trait     | Description | Usage |
-------   | ----------- | ----- |
[AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) | For a type `T`, get a `&X` cheaply | Making function parameters more flexible (pseudo-overloading) |   
[AsMut](https://doc.rust-lang.org/std/convert/trait.AsMut.html) | Mutable version of `AsRef` | |
[Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) | For a type `T`, get a `&X` cheaply that has the same equality semantics | Make type `T` work well as a key in associative containers |
[BorrowMut](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html) | Mutable version of `Borrow` | |

`AsRef` is often used in a similar role as `Into`, for making function parameters accept a wider range of types. How to
choose between them?

* When there are a lot of types for which `AsRef` can be reasonably implemented, favour `AsRef`, because it is cheap.
* When conversions require some work to be done, use `Into`.

`Borrow<T>` is like `AsRef<T>` but goes further: if a type `X` implements `Borrow<T>` the borrowed `&T` is expected to
hash (trait = [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)) and equality-compare
(trait = [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)) the same was as the original type `X`. This allows the
borrowed reference to perform as a 'susbtitute' for `X` when used as a key in associative containers such as
[HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) - the conditions described ensure that the 
`X` and the `&T` borrowed from it will always identify the same element in the container. If you implement `Borrow<T>`
for your type, it's up to you to ensure you uphold the promise.

<p>&nbsp;</p>

## Traits for smart pointer types

Trait    | Description | Usage |
-------  | ----------- | ----- |
[Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) | For `&T`, get a reference to some `&T::Target` | Creating smart pointers; automatic reference conversion |  
[DeRefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) | Mutable version of `DeRef` | |
[Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html) | Clone-on-write smart pointer | For delaying and optimising allocations |

`DeRef` and `DeRefMut` are used to create 'transparent containers' which allow you to call methods on the thing inside the
container using a reference to the container instead of a reference to the thing. They affect the behaviour of the
`*` and `.` (de-referencing) operators.

For example the type [Box&lt;T&gt;](https://doc.rust-lang.org/std/boxed/struct.Box.html), a pointer into the heap,
implements `DeRef<Target=T>`, so if you have a `b` of type `Box<&str>` then you can
call `&str` methods such as [is_empty](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
and [lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
directly. without having to de-reference the `b` first:

```rs
let b: Box<&str> = Box::new("hello world");

if b.is_empty() {
    // do something
}
```   

`Cow` is actually an enum rather than a trait, but is mentioned here because it is also used for creating smart pointers.
The point of `Cow` is to allow you to defer until runtime the decision to clone the value in the Cow. For example,
if you never change the value then you can just continue to borrow it, saving an allocation. But if at some point you
decide to mutate it (e.g. convert a `&str` to lowercase) then the `Clone` will happen automatically. 

<p>&nbsp;</p>

## Traits for comparisons

Trait      | Description | Usage |
---------  | ----------- | ----- |
[PartialEq*](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) | The type of equality when it's possible two values from your domain *can't* be compared | Floating point types |
[Eq*](https://doc.rust-lang.org/std/cmp/trait.Eq.html) | The type of equality when all values in your domain *can* be compared | Pretty much everything NOT a floating point type |
[PartialOrd*](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) | The type of ordering when it's possible two values from your domain *can't* be compared | Floating point types |
[Ord*](https://doc.rust-lang.org/std/cmp/trait.Ord.html)| The type of ordering when all values in your domain *can* be compared | Pretty much everything NOT a floating point type |
[Hash*](https://doc.rust-lang.org/std/hash/trait.Hash.html) | Identify types which can be hashed | Such types can be used as keys in associative collections |

This set of traits is for expressing the concepts of *equality* of two values (i.e. the operations `==` and `!=`) and
the *ordering* of two values (the other relational operators, `<`, `<=`, `>=`, `>`).

See also Operator Overloading.

<p>&nbsp;</p>

## Traits for dealing with numbers

Trait    | Description | Usage |
-------  | ----------- | ----- |
[Num](https://docs.rs/num/0.1.42/num/trait.Num.html) | A trait for identifying numbers | Useful in generic bounds |  
[Integer](https://docs.rs/num/0.1.42/num/trait.Integer.html)| A trait for identifying integers | Useful in generic bounds |
[Float](https://docs.rs/num/0.1.42/num/trait.Float.html) | A trait for identifying floating point numbers | Useful in generic bounds |
[Unsigned](https://docs.rs/num/0.1.42/num/trait.Unsigned.html) | A trait for numbers which cannot be negative | Useful in generic bounds |
[Signed](https://docs.rs/num/0.1.42/num/trait.Signed.html) | A trait for numbers which can be positive or negative | Useful in generic bounds |

These traits are not built-in to the Rust standard library, but are available in the widely-used
[num crate](https://crates.io/crates/num) from [crates.io](https://crates.io/).

<p>&nbsp;</p>

## Traits for indexing into collections

Trait    | Description | Usage |
-------- | ----------- | ----- |
[Index](https://doc.rust-lang.org/std/ops/trait.Index.html) | Make the read-only indexing operator `a = x.[n]` available | Implement on custom collection types | 
[IndexMut](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) | Make the writable indexing operator `x.[n] = a` available | Implement on custom collection types |

<p>&nbsp;</p>

## Traits for formatting

Trait   | Description | Usage |
------- | ----------- | ----- |
[Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) | Convert a value to a formatted string | Intended for human-friendly output | 
[Debug*](https://doc.rust-lang.org/std/fmt/trait.Debug.html) | Convert a value to a formatted string | Intended for programmers, debugging etc. |

TODO: Say something about round-tripping Display, and the From/Into/FromStr traits.

<p>&nbsp;</p>

## Traits for threading

Trait | Description | Usage |
------| ----------- | ----- |
[Send](https://doc.rust-lang.org/std/marker/trait.Send.html)  | Specifies that a type can be moved between threads | Used to pass values from one thread to another, often to mutate them |
[Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) | Specifies that references to a type can be shared between threads | Used to share immutable state between threads (*) |

These traits are used in multi-threading scenarios: they specify how values can be passed between threads. The compiler
implements them automatically by examining the fields in your structs. They are another example of Rust eliminating
unsafe behaviour - together, they basically guarantee that if your multi-threading code compiles it has no
undefined behaviour such as data races. 

(*) Or state that is protected by interior mutability.

<p>&nbsp;</p>

## Traits for IO

Trait   | Description | Usage |
------- | ----------- | ----- |
[Write](https://doc.rust-lang.org/std/io/trait.Write.html) | For types which you can write bytes to | Examples: files, vectors, strings, network streams |   
[Read](https://doc.rust-lang.org/std/io/trait.Read.html) | For types which you can read bytes from | Examples: files, slices, network streams |
[BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html) | For types which wrap readers and buffer read calls | Speed up reading |
[BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html) | A standard implementation of `BufRead` | Speed up reading |
[BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html) | A standard implementation of a type which wraps a `Write`r and buffers write calls | Speed up writing |
[Seek](https://doc.rust-lang.org/std/io/trait.Seek.html) | For types which allow you to move backwards and forwards in the stream of bytes | Examples: files, slices, `BufReader`, `BufWriter` |

`BufReader` and `BufWriter` are actually structs rather than traits, but they are mentioned here for completeness and you
should use them as best-practice.

## Misc
Trait   | Description | Usage |
------- | ----------- | ----- |
[Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html) | Specifies that a value has a known, fixed size | Implemented by default for the vast majority of Rust types |
[Error](https://doc.rust-lang.org/std/error/index.html) | The base trait for all error types | Handling errors generically; defining your own error types |

