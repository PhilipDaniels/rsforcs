+++
title = "Commonly Used Traits"
weight = 10
sort_by = "weight"
+++

A *trait* is something like an interface - a grouping of functions that together serve a circumscribed purpose. In Rust,
there is a small set of traits which are very commonly used - you are almost sure to use at least one of them in any
program you write, so being familiar with them is a necessity. Luckily, most of them are very simple, consisting of
only one function!

Here is a summary. Traits that can be auto-derived are marked with a `*`.

<p>&nbsp;</p>

## Misc
Trait   | Description | Usage |
------- | ----------- | ----- |
Sized | Specifies that a value has a known, fixed size | Implemented by default by the vast majority of Rust types |
Error | The base trait for all error types | Handling errors generically; defining your own error types |

<p>&nbsp;</p>

## Traits relating to the lifecycle of a value

Trait   | Description | Usage |
------- | ----------- | ----- |
Default | For creating default values | To simplify creation of large structs; when a default is needed in generic code |
Clone   | For making independent copies of a value | 'Independent' usually implies making new heap allocations so this can be expensive | 
Copy    | Marker trait for types that can be bit-copied | An extension of `Clone`, only for types that do not own resources |
Drop    | Destructor. | For when you need custom clean-up code |

<p>&nbsp;</p>

## Traits for converting values

Trait   | Description | Usage |
------- | ----------- | ----- |
Into    | For type X, take ownership of X-value and convert to type T | Making function parameters more flexible (pseudo-overloading) |
From    | For type X, convert to type T | Generic way of writing single-argument constructors |
TryFrom | Version of `From` to use if the conversion can fail | Nightly-only |
TryInto | Version of `Into` to use if the conversion can fail | Nightly-only |
ToOwned | Make an owned value from a reference | When `Clone` is not flexible enough |
FromStr | Make a value by parsing a `&str` | A more specialized version of `From` that can return an error |
FromIterator | Build a value from the items in an iterator | Make it easy to construct specialist collection types |

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
AsRef     | For a type `T`, get a `&X` cheaply | Making function parameters more flexible (pseudo-overloading) |   
AsRefMut  | Mutable version of `AsRef` | |
Borrow    | For a type `T`, get a `&X` cheaply that has the same equality semantics | Make type `T` work well as a key in associative containers |
BorrowMut | Mutable version of `Borrow` | |

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
Deref    | For `&T`, get a reference to some `&T::Target` | Creating smart pointers; automatic reference conversion |  
DeRefMut | Mutable version of `DeRef` | |

These two traits are used to create 'transparent containers' which allow you to call methods on the thing inside the
container using a reference to the container instead of a reference to the thing. They affect the behaviour of the
`*` and `.` (de-referencing) operators.

For example the type [Box&lt;T&gt;](https://doc.rust-lang.org/std/boxed/struct.Box.html), a pointer into the heap,
implements `DeRef<Target=T>`, so if you have a `b` of type `Box<&str>` then you can
call `&str` methods such as [is_empty](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
and [lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
directly:

```rs
let b: Box<&str> = Box::new("hello world");

if b.is_empty() {
    // do something
}
```   

<p>&nbsp;</p>

## Traits for comparisons

Trait      | Description | Usage |
---------  | ----------- | ----- |
PartialEq  | The type of equality when it's possible two values from your domain *can't* be compared | Floating point types |
Eq         | The type of equality when all values in your domain *can* be compared | Pretty much everything NOT a floating point type |
PartialOrd | The type of ordering when it's possible two values from your domain *can't* be compared | Floating point types |
Ord        | The type of ordering when all values in your domain *can* be compared | Pretty much everything NOT a floating point type |
Hash       |

This set of traits is for expressing the concepts of *equality* of two values (i.e. the operations `==` and `!=`) and
the *ordering* of two values (the other relational operators, `<`, `<=`, `>=`, `>`).

See also Operator Overloading.

<p>&nbsp;</p>

## Traits for dealing with numbers

Trait    | Description | Usage |
-------  | ----------- | ----- |
Num      | A trait for identifying numbers | Useful in generic bounds |  
Integer  | A trait for identifying integers | Useful in generic bounds |
Float    | A trait for identifying floating point numbers | Useful in generic bounds |
Unsigned | A trait for numbers which cannot be negative | Useful in generic bounds |

<p>&nbsp;</p>

## Traits for indexing into collections

Trait    | Description | Usage |
-------- | ----------- | ----- |
Index    | 
IndexMut | 

<p>&nbsp;</p>

## Traits for formatting

Trait   | Description | Usage |
------- | ----------- | ----- |
Display | 
Debug   | 

<p>&nbsp;</p>

## Traits for threading

Trait | Description | Usage |
------| ----------- | ----- |
Send  | 
Sync  | 

<p>&nbsp;</p>

## Traits for IO

Trait   | Description | Usage |
------- | ----------- | ----- |
Write   |  
Read    |  
BufRead |
Seek    |

