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

`Option` often shows up as a member of a struct or the return type of a function. The other very frequently used type in
the function situation is the `Result<T, E>` type, which I always think of as "an option, but you can specify error
information instead of just `None`". See the [Result&lt;T, E&gt;](./basics/result.md) page for more. When writing a
function:

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

Method | Description |
------ | ----------- | 
<thead class="subhead"><th colspan="2">Get the value</th></thead> 
[is_some(&self) -> bool](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some) | Check if the option has a value, i.e. it's a `Some`
[is_none(&self) -> bool](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none) | Check if the option doesn't have a value, i.e. it's a `None` <thead class="subhead"><th colspan="2">Get the value</th></thead>
[unwrap(self) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) | Returns the value - moves it out of the option. Panics if the option is a `None`.
[expect(self, msg: &str) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect) | Like `unwrap`, but allows you to specify a custom panic message.
[unwrap_or(self, def: T) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) | Like `unwrap`, but you can specify a default value to be returned if the option is a `None` instead of panicking
[unwrap_or_else&lt;F&gt;(self, f: F) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) | Like `unwrap`, but you can specify a function to call if the option is a `None` instead of panicking
[unwrap_or_default(self) -> T](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default) | Like `unwrap`, but returns the [Default](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default) value if the option is a `None` instead of panicking
[iter(&self) -> Iter&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter) | Get an iterator over the value; returns a max of 1 item
[iter_mut(&mut self) -> IterMut&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter_mut) | Get a mutable iterator over the value; returns a max of 1 item
[take(&mut self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) | Take the value out of the option, leaving a None in its place. <thead class="subhead"><th colspan="2">Boolean combinators</th></thead>
[or(self, optb: Option&lt;T&gt;) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.or) | Logical "OR" - return first option, or second if the first is `None`
[and&lt;U&gt;(self, optb: Option&lt;U&gt;) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.and) | Logical "AND" - return `None` if the first is `None`, else return the second option <thead class="subhead"><th colspan="2">Transform the value</th></thead>
[map&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) | If `Some`, apply `f` to it and return the result, else return `None`
[map_or&lt;U, F&gt;(self, default: U, f: F) -> U](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) | If `Some`, apply `f` to it and return the result, else return the `default` value
[map_or_else&lt;U, D, F&gt;(self, default: D, f: F) -> U](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else) | If `Some`, apply `f` to it and return the result, else return the result of calling the function `default` <thead class="subhead"><th colspan="2">Chaining</th></thead>
[and_then&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) | If `None`, returns `None`, else returns the result of calling `f` with the value 
[or_else&lt;F&gt;(self, f: F) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else) | If `Some` returns the value, else returns the result of calling the function `f` <thead class="subhead"><th colspan="2">Get reference</th></thead>
[as_ref(&self) -> Option&lt;&T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) | Converts from `Option<T>` to `Option<&T>`
[as_mut(&mut self) -> Option&lt;&mut T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut) | Converts from `Option<T>` to `Option<&mut T>` <thead class="subhead"><th colspan="2">Convert to Result</th></thead>
[ok_or&lt;E&gt;(self, err: E) -> Result&lt;T, E&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) | Create a `Result` from this option. If `Some(v)` return `Ok(v)`, else return `Err(err)` (where err is a value)
[ok_or_else&lt;E, F&gt;(self, err: F) -> Result&lt;T, E&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else) | Create a `Result` from this option. If `Some(v)` return `Ok(v)`, else return `Err(err())` (where err is a function) <thead class="subhead"><th colspan="2">Set the value</th></thead>
[get_or_insert(&mut self, v: T) -> &mut T](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert) | If `None`, set the value to `v`, else leave it alone. Then return a mutable reference to the value. 
[get_or_insert_with&lt;F&gt;(&mut self, f: F) -> &mut T](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with) | If `None`, set the value to the result of calling `f`, else leave it alone. Then return a mutable reference to the value. <thead class="subhead"><th colspan="2">Clone</th></thead>
[cloned(self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned) | If `T` is cloneable, convert an `Option<&T>` to an `Option<T>` by cloning the contents of the option.
[cloned(self) -> Option&lt;T&gt;](https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned-1) | If `T` is cloneable, convert an `Option<&mut T>` to an `Option<T>` by cloning the contents of the option. <thead class="subhead"><th colspan="2">Comparisons</th></thead>
[PartialOrd](https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialOrd%3COption%3CT%3E%3E), [Ord](https://doc.rust-lang.org/std/option/enum.Option.html#impl-Ord), [Eq](https://doc.rust-lang.org/std/option/enum.Option.html#impl-Eq), [PartialEq](https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialEq%3COption%3CT%3E%3E) | `Option<T>` supports the standard Rust relational operators *if* the `T` supports them |


**n.b.** Function signatures may be abbreviated, for example generic bounds might be elided. Click through for the full details.

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

<table>
    <thead><th>Option&lt;T&gt; Method</th><th>Result&lt;T, E&gt; Method</th><th>Description</th></thead>
    <thead class="subhead"><th colspan="3">Testing</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some">is_some(&self) -> bool</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok">is_ok(&self) -> bool</a> </td>
            <td> Return true if the value is <code>Some / Ok</code> </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none">is_none(&self) -> bool</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err">is_err(&self) -> bool</a> </td>
            <td> Return true if the value is <code>None / Err</code> </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Getting the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap">unwrap(self) -> T</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap">unwrap(self) -> T</a> </td>
            <td> Returns the value - moves it out of the option. Panics if the option is a <code>None</code> </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.expect">expect(self, msg: &str) -> T</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.expect">expect(self, msg: &str) -> T</a> </td>
            <td> Like <code>unwrap</code>, but allows you to specify a custom panic message. </td>
        </tr>
        <tr>
            <td>  </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err">unwrap_err(self) -> E</a> </td>
            <td> Returns the err - moves it out of the result. Panics if the result is an <code>Ok</code> </td>
        </tr>
        <tr>
            <td>  </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err">expect_err(self, msg: &str) -> E</a> </td>
            <td> Like <code>unwrap_err</code>, but allows you to specify a custom panic message. </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or">unwrap_or(self, def: T) -> T</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or">unwrap_or(self, optb: T) -> T</a> </td>
            <td> Like <code>unwrap</code>, but you can specify a default value to be returned if the option is a <code>None</code> instead of panicking </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else">unwrap_or_else&lt;F&gt;(self, f: F) -> T</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else">unwrap_or_else&lt;F&gt;(self, op: F) -> T</a> </td>
            <td> Like <code>unwrap</code>, but you can specify a function to call if the option is a <code>None</code> instead of panicking </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default">unwrap_or_default(self) -> T</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default">unwrap_or_default(self) -> T</a> </td>
            <td> Like <code>unwrap</code>, but returns the <a href=https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default">Default</a> value if the option is a <code>None</code> instead of panicking </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.iter">iter(&self) -> Iter&lt;T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.iter">iter(&self) -> Iter&lt;T&gt;</a> </td>
            <td> Get an iterator over the value; returns a max of 1 item </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.iter_mut">iter_mut(&mut self) -> IterMut&lt;T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.iter_mut">iter_mut(&mut self) -> IterMut&lt;T&gt;</a> </td>
            <td> Get a mutable iterator over the value; returns a max of 1 item </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.take">take(&mut self) -> Option&lt;T&gt;</a> </td>
            <td> </td>
            <td>Take the value out of the option, leaving a <code>None</code> in its place.</td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Getting a reference to the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref">as_ref(&self) -> Option&lt;&T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref">as_ref(&self) -> Result<&T, &E></a> </td>
            <td> Converts from <code>Option<T></code> to <code>Option<&T></code> </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut">as_mut(&mut self) -> Option&lt;&mut T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.as_mut">as_mut(&self) -> Result<&mut T, &mut E></a> </td>
            <td> Converts from <code>Option<T></code> to <code>Option<&mut T></code> </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Transforming the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.map">map&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map">map<U, F>(self, op: F) -> Result&lt;U, E&gt; </a> </td>
            <td> If <code>Some</code>, apply <code>f</code> to it and return the result, else return <code>None</code> </td>
        </tr>
        <tr>
            <td> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">map_err&lt;F, O&gt;(self, op: O) -> Result&lt;T, F&gt;  </a> </td>
            <td> Apply the function <code>op</code> to any <code>Err</code> and return the result, pass <code>Ok</code> values through unchanged </td>
        </tr>        
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or">map_or&lt;U, F&gt;(self, default: U, f: F) -> U</a> </td>
            <td> </td>
            <td> If <code>Some</code>, apply <code>f</code> to it and return the result, else return the <code>default</code> value </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else">map_or_else&lt;U, D, F&gt;(self, default: D, f: F) -> U</a> </td>
            <td> </td>
            <td> If <code>Some</code>, apply <code>f</code> to it and return the result, else return the result of calling the function <code>default</code> </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Cloning the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-Clone">clone(&self) -> Option&lt;T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-Clone">clone(&self) -> Result&lt;T, E&gt;</a> </td>
            <td> If <code>T</code> (and <code>E</code> for Results) is cloneable, return a copy of the value </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-Clone">clone_from(&mut self, source: &Self)</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-Clone">clone_from(&mut self, source: &Self)</a> </td>
            <td> Performs copy-assignment from <code>source</code> </td>
        </tr>        
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned">cloned(self) -> Option&lt;T&gt;</a> </td>
            <td> </td>
            <td>If <code>T</code> is cloneable, convert an <code>Option<&T></code> to an <code>Option<T></code> by cloning the contents of the option.</td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned-1">cloned(self) -> Option&lt;T&gt;</a> </td>
            <td> </td>
            <td> If <code>T</code> is cloneable, convert an <code>Option<&mut T></code> to an <code>Option<T></code> by cloning the contents of the option.</td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Combining two Options or Results logically into a new Option or Result</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.or">or(self, optb: Option&lt;T&gt;) -> Option&lt;T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.or">or(self, res: Result&lt;T, F&gt;) -> Result&lt;T, F&gt;</a> </td>
            <td> Logical "OR" - If first is <code>Some</code> or <code>Ok</code> return it, else return the second </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.and">and&lt;U&gt;(self, optb: Option&lt;U&gt;) -> Option&lt;U&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.and">and&lt;U&gt;(self, res: Result&lt;U, E&gt;) -> Result&lt;U, E&gt;</a> </td>
            <td> Logical "AND" - If first is <code>None</code> or <code>Err</code> return first, else return the second </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Selectively calling functions based on the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then">and_then&lt;U, F&gt;(self, f: F) -> Option&lt;U&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then">and_then&lt;U, F&gt;(self, op: F) -> Result&lt;U, E&gt;</a> </td>
            <td> If value is <code>Some</code> or <code>Ok</code> call the <code>f</code> on it, else pass the <code>None</code> or <code>Err</code> through unchanged </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else">or_else&lt;F&gt;(self, f: F) -> Option&lt;T&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.or_else">or_else&lt;F, O&gt;(self, op: O) -> Result&lt;T, F&gt;</a> </td>
            <td> If value is <code>Some</code> or <code>Ok</code> return it unchanged, else call <code>f</code> and return the result. For Results, <code>f</code> is passed the <code>Err</code> </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Converting Option &rArr; Result and Result &rArr; Option</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or">ok_or&lt;E&gt;(self, err: E) -> Result&lt;T, E&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.ok">ok(self) -> Option&lt;T&gt;</a> </td>
            <td> Create a <code>Result</code> from this option. If <code>Some(v)</code> return <code>Ok(v)</code>, else return <code>Err(err)</code> (where err is a value) </td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else">ok_or_else&lt;E, F&gt;(self, err: F) -> Result&lt;T, E&gt;</a> </td>
            <td> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.err">err(self) -> Option&lt;E&gt;</a> </td>
            <td> Create a <code>Result</code> from this option. If <code>Some(v)</code> return <code>Ok(v)</code>, else return <code>Err(err())</code> (where err is a function) </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Setting the value</th></thead>
    <tbody>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert">get_or_insert(&mut self, v: T) -> &mut T</a> </td>
            <td> </td>
            <td>If <code>None</code>, set the value to <code>v</code>, else leave it alone. Then return a mutable reference to the value.</td>
        </tr>
        <tr>
            <td> <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with">get_or_insert_with&lt;F&gt;(&mut self, f: F) -> &mut T</a> </td>
            <td> </td>
            <td>If <code>None</code>, set the value to the result of calling <code>f</code>, else leave it alone. Then return a mutable reference to the value. </td>
        </tr>
    </tbody>
    <thead class="subhead"><th colspan="3">Comparing</th></thead>
    <tbody>
        <tr>
            <td>
                <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialOrd%3COption%3CT%3E%3E">PartialOrd</a>,
                <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-Ord">Ord</a>,
                <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-PartialEq%3COption%3CT%3E%3E">PartialEq</a>,
                <a href="https://doc.rust-lang.org/std/option/enum.Option.html#impl-Eq">Eq</a>
            </td>
            <td>
                <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-PartialOrd%3CResult%3CT%2C%20E%3E%3E">PartialOrd</a>,
                <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-Ord">Ord</a>,
                <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-PartialEq%3CResult%3CT%2C%20E%3E%3E">PartialEq</a>,
                <a href="https://doc.rust-lang.org/std/result/enum.Result.html#impl-Eq">Eq</a>
            </td>
            <td> If <code>T</code> (and <code>E</code> for Results) supports the standard Rust relational operators then the <code>Option</code> or <code>Result</code> also supports them </td>
        </tr>
    </tbody>
</table>

