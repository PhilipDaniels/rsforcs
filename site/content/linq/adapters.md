+++
title = "Writing Your Own Iterator Adapters"
weight = 5
+++

An *iterator adapter* is a function which you can call on an iterator, producing either a value or
another iterator. This article describes how to write your own custom adapters. When writing an
adapter, we are trying to achieve the following syntax:

```rs
collection.iter().map(...).myadapter().filter()...
```

That is, a function which can be slotted into an existing iterator function call chain, as opposed
to just using a free function:

```rs
myadapter(collection.iter().map(...)).filter()...
```

The former syntax is a lot more natural. In C# terms, what we are trying to achieve is to add an
*extension method* to Rust's [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
trait. To do this, first we define a trait, then we `impl` the trait on *Iterator*. Therefore all
our adapters will have at least two components:

- A trait which extends Iterator
- An impl block which adds the functions of that trait to Iterator

That's enough for adapters which do not need to manage state, but more complex adapters will need to
track state between calls to their `next` method. This is handled by the third step:

- Define a struct to manage iterator state

This third step is why if you look at the documentation for
[Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) you can see so many structs
defined - there is one for each iterator method which needs to manage some internal state.


## Adapters without state

Let's create an iterator adapter for `Single`. This returns the item if and only if there is one
match, `None` otherwise:

```rs
/// First define the trait. It has only a single function. `Self::Item` is the type of the values
/// being iterated over. This adapter returns only a single value (not a sequence) and has no
/// need of any state.
trait SingleIteratorAdapter : Iterator {
    fn single(&mut self) -> Option<Self::Item>
{
        match self.next() {
            None => None,
            Some(x) => match self.next() {
                None => Some(x),
                Some(_) => None
            }
        }
    }
}

/// What this says is "I want to add the methods in the trait SingleIteratorAdapter to the trait `I`".
/// `I` happens to be any type of Iterator, so we are effectively writing an extension method that
/// works against all types of iterators.
impl<I> SingleIteratorAdapter for I where I : Iterator {
    // Normally an impl block would contains functions here. But in this case, there is only
    // one function and it already has a default definition in the trait, so we don't have to
    // write anything!
}
```

## Adapters with state

For these adapters, we need to define a struct then

```rs

```