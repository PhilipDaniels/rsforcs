+++
title = "Null is Optional"
weight = 20
+++

## Never null, never fear

Rust references are never null. There is no need to write `if x != null` checks as argument guards or program logic,
and consequently you will never see a `NullReferenceException` - not least because Rust does not have exceptions.

C# can be described as having a 3-legged type system (although unified of course, because everything ultimately
derives from `System.Object`).

* There are
[value](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/value-types) types, which constitute
the fundamental types such as `int` and `bool`, and user-defined
[structs](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/struct).
* Then there are [reference](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/reference-types)
types, which is everything else, especially classes.

C# programmers could always represent the presence or absence of a reference type by using a `null` because reference
types are allocated on the heap, so a null pointer is easy to add to the type system. There was no way
of reliably(*) indicating presence/absence for value types, so we have the third leg:

* [System.Nullable&lt;T&gt;](https://msdn.microsoft.com/en-us/library/b3h38hb0(v=vs.110).aspx) was
introduced in .Net 2.0. It basically wraps a value type and adds a boolean flag to say whether the value is present or
not.


(*) One approach is to use sentinel values such as -1, for example the
[String.IndexOf()](https://msdn.microsoft.com/en-us/library/k8b1470s(v=vs.110).aspx) method does this. That works OK
for this method because -1 can never be a valid string index, but if any int is a valid return value this
approach breaks down.

## Rust's answer to the present/absent question

Interestingly, `Nullable<T>` is very close to the Rust solution to the problem of modelling "either we have a value, or
we don't" - that solution is called [Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html). It's
ubiquitous in Rust code. The definition is engagingly simple:
 
```rs
pub enum Option<T> {
    None,
    Some(T),
}
```

`Option` often shows up as the return type of a function. The other very frequently used type in that situation is the
`Result<T, E>` type, which I always think of as "an option, but you can specify error information instead of just `None`".
See the [Result&lt;T, E&gt;](./basics/result.md) page for more. When writing a function:

> If your function might return a value or not but **definitely won't** throw an error, use `Option<T>`
>
> If your function will return a value or **possibly** an error, use `Result<T, E>`
>
> If your function has nothing of interest to return but **might** return an error, use `Result<(), E>`

The difference between `Nullable<T>` and `Option<T>` is that in Rust the compiler helps you use the value correctly.
While it is still possible to shoot yourself in the foot, it's not so easy. If you want to use the value inside the
`Some(T)` you have to deconstruct the option, either using pattern matching or one of the methods it provides for
getting at the value.

Pattern matching first:

```rs
// Create an option that has a value. Uses type inference, this is an Option<i32>
let opt = Some(42);

// In the following match expression, n is bound to the value in the Option. But this
// only happens if the Option is Some() - so in the None branch we can't even refer to n!
// That is how we avoid accidentally de-referencing a null reference.
match opt {
    Some(n) => println!("The answer is {}", n),     // Prints "The answer is 42"
    None => println!("There is no value")
}

// If you having nothing to do in the None branch, you can use `if let` instead:
if let Some(n) = opt {
    println!("The answer is {}", n);     // Prints "The answer is 42"
}

// Rust is able to figure out you want an Option with a specific `T` from the `Some()` syntax.
// But if you want a None, you have to help out the type system in either of these ways:
let x = Option::<u32>::None;
let x: Option<u32> = None;
```

As you can see, creating options is easy - just write `Some(the_value)`. 

> Aside: Rust's [Iterator.next()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
> function returns `None` to signal the end of a sequence, rather than having
> [separate](https://msdn.microsoft.com/en-us/library/78dfe2yb.aspx)
> `get_current_item/move_next` methods.
 
## So you have an Option<T>, what can you do with it?

What | Method | Description |
---- | ------ | ----------- |
Check | [is_some(&self) -> bool](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some) | Check if the option has a value, i.e. it's a `Some`
Check | [is_none(&self) -> bool](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none) | Check if the option doesn't have a value, i.e. it's a `None` <thead><th></th><th></th><th></th></thead>
Get the value | [unwrap(self) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) | Returns the value - moves it out of the option. Panics if the option is a `None`.
Get the value | [expect(self, msg: &str) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect) | Like `unwrap`, but allows you to specify a custom panic message.
Get the value | [unwrap_or(self, def: T) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) | Like `unwrap`, but you can specify a default value to be returned if the option is a `None` instead of panicking
Get the value | [unwrap_or_else&lt;F&gt;(self, f: F) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) | Like `unwrap`, but you can specify a function to call if the option is a `None` instead of panicking
Get the value | [unwrap_or_default(self) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default) | Like `unwrap`, but returns the [Default](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default) value if the option is a `None` instead of panicking
Get the value | [iter(&self) -> Iter&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter) | Get an iterator over the value; returns a max of 1 item
Get the value | [iter_mut(&mut self) -> IterMut&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter_mut) | Get a mutable iterator over the value; returns a max of 1 item
Get the value | [take(&mut self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) | Take the value out of the option, leaving a None in its place. <thead><th></th><th></th><th></th></thead>
Boolean logic | [or(self, optb: Option&lt;T&gt;) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.or) | Logical "OR" - return first option, or second if the first is `None`
Boolean logic | [and&lt;U&gt;(self, optb: Option&lt;U&gt;) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.and) | Logical "AND" - return `None` if the first is `None`, else return the second option <thead><th></th><th></th><th></th></thead>
Transform the value | [map&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) | If `Some`, apply `f` to it and return the result, else return `None`
Transform the value | [map_or&lt;U, F&gt;(self, default: U, f: F) -> U](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) | If `Some`, apply `f` to it and return the result, else return the `default` value
Transform the value | [map_or_else&lt;U, D, F&gt;(self, default: D, f: F) -> U](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else) | If `Some`, apply `f` to it and return the result, else return the result of calling the function `default` <thead><th></th><th></th><th></th></thead>
Chaining | [and_then&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) | If `None`, returns `None`, else returns the result of calling `f` with the value 
Chaining | [or_else&lt;F&gt;(self, f: F) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else) | If `Some` returns the value, else returns the result of calling the function `f` <thead><th></th><th></th><th></th></thead>
Get reference | [as_ref(&self) -> Option&lt;&T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) | Converts from `Option<T>` to `Option<&T>`
Get reference | [as_mut(&mut self) -> Option&lt;&mut T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut) | Converts from `Option<T>` to `Option<&mut T>` <thead><th></th><th></th><th></th></thead>
Convert to Result | [ok_or&lt;E&gt;(self, err: E) -> Result&lt;T, E&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) | Create a `Result` from this option. If `Some(v)` return `Ok(v)`, else return `Err(err)` (where err is a value)
Convert to Result | [ok_or_else&lt;E, F&gt;(self, err: F) -> Result&lt;T, E&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else) | Create a `Result` from this option. If `Some(v)` return `Ok(v)`, else return `Err(err())` (where err is a function) <thead><th></th><th></th><th></th></thead>
Set the value | [get_or_insert(&mut self, v: T) -> &mut T](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert) | If `None`, set the value to `v`, else leave it alone. Then return a mutable reference to the value. 
Set the value | [get_or_insert_with&lt;F&gt;(&mut self, f: F) -> &mut T](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with) | If `None`, set the value to the result of calling `f`, else leave it alone. Then return a mutable reference to the value. <thead><th></th><th></th><th></th></thead>
Clone | [cloned(self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned) | If `T` is cloneable, convert an `Option<&T>` to an `Option<T>` by cloning the contents of the option.
Clone | [cloned(self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned-1) | If `T` is cloneable, convert an `Option<&mut T>` to an `Option<T>` by cloning the contents of the option. <thead><th></th><th></th><th></th></thead>
Comparisons | [PartialOrd](https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialOrd%3COption%3CT%3E%3E), [Ord](https://doc.rust-lang.org/std/option/enum.Option.html#impl-Ord), [Eq](https://doc.rust-lang.org/std/option/enum.Option.html#impl-Eq), [PartialEq](https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialEq%3COption%3CT%3E%3E) | `Option<T>` supports the standard Rust relational operators *if* the `T` supports them |


**n.b.** Function signatures are incomplete, bounds not shown etc. Click through for the full details.

* `unwrap` is used frequently, despite its possibility of panicking. For demo code, or cases where you know (based on
  the inputs) that the value can never actually be a `None`, it's fine.
* `unwrap_or...` variants are Rust's way of expressing 'thing or default'. Again, you will see them a lot.
* `expect` is basically unwrap but with a friendlier error message
* The difference between the `unwrap` variants and `take` is that `unwrap` and friends **move** out of the option,
  leaving it unusable. `Take` replaces the value with a `None`, which means the option is still usable in further code.
* Most of these methods consume the option. Sometimes you just want to peek inside. That's where `as_ref` and
  `as_mut` come in.
* Notice the naming convention: functions ending with `_or` allow you to supply a default value for use when the option
  is `None`. The functions ending with `_or_else` allow you to supply a function for the same purpose.
* The `or` and `and` functions allow you to logically combine options without unpacking them first.  
* `and_then` can be useful in logic chains - it will only call the function you supply if the value is a `Some`, so you
  don't need to unpack the option first (say with an `if let`). In other words, a `None` can be made to safely flow
  through code. See examples below.
* The `ok_or` and `ok_or_else` functions for converting to a `Result<T, E>` have corresponding functions on `Result`
  for converting the other way, to an `Option`.

## Idiomatic use of Options

Use with collections of things

Logic chains

Comparisons

https://blog.burntsushi.net/rust-error-handling/#composing-option-and-result

## See Also

The big Option/Result method table.

