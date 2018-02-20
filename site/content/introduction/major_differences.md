+++
title = "Major differences between Rust and C#"
weight = 20
+++

## Introduction

The purpose of this page is to give a signpost to some of the major differences between C# and Rust.
Individual topics will be treated in more detail later on.

## Similar, but different

C# and Rust are both portable, statically typed, imperative languages with functional aspects and
generics. But in terms of the runtime model, they are vastly different. A C# program consists of
Intermediate Language (IL) that cannot be directly executed by the processor. At execution time, the
CLR compiles this meta-code into actual CPU instructions using the Just-in-Time compiler (the
JITter). The CLR also provides a rich runtime environment, featuring such things as AppDomains,
Reflection, Garbage Collection and Security. Much of this is possible because a C# assembly contains
not only the IL but extensive meta data about the types the assembly contains and uses. See CLR via
C# by Richter for more details.

In contrast, a Rust executable is a much simpler thing, it has static data and native machine code
instructions. That's it. The executable code is produced by the Rust compiler at the time the
program is built (called ahead-of-time compilation) and during that compilation step all information
about the high level types in your program is eliminated. What you are left with is data bytes and
program bytes. A Rust executable is therefore the same in nature as an executable produced by a C,
Ada or C++ compiler.

The fact that Rust is compiled ahead-of-time, and uses the phenomenal LLVM compiler as a back-end,
means that Rust programs should run faster than the equivalent C# program. It's simply a fact that
the Rust compiler can spend a lot more time optimising your program - minutes if it wants to -
whereas the CLR JIT compiler must produce the machine code in milli- or micro-seconds in order to
have good cold-start performance.

> The ability to write high-level C#-like programs that have native performance is one of Rust's
> most attractive features.

Rust executables are statically linked<sup>(1)</sup> - this means that the exe contains all the code and
libraries necessary for it to execute, so you have true xcopy deployment - you can simply copy the
exe to another machine and it will run. There is no equivalent to "you must install the .Net
Framework 4.6.2 in order to run this program." The downside to AOT compilation is that if you want
to distribute your program in executable form you must produce an exe for each target architecture,
because the exe will contain machine code instructions specific to each one. So for example, you
might need to build exes for Windows, Linux and MacOS. See the TODO BUILD section for more
information.

(1) It is possible to produce dynamically linked executables if desired, and if you call out to
native DLLs then you must ensure they exist as well.

## Rust does not have garbage collection, it has ownership

The biggest difference between C# and Rust, the one that will impact you most in the way that you
design programs - and probably frustrate you most - is the way that you compose programs from their
constituent parts.

In C#, design consists of creating types (usually classes) that encapsulate data and code
(properties and methods). Then we compose programs by linking these types together using references
(embedding a class in another class as a member variable is just another type of linking). It's easy
to build complex functionality by continuing this process to any degree - "I need an X here? Fine,
just add another constructor parameter or a property". Dependency injection even lets a library take
care of constructing our object hierarchies "automatically" for us. This leads to a mental model
where one can focus on the design of an individual type, and not worry too much about how it will
eventually fit into the overall program, or even its immediate parents. This is a highly productive
way of working, and it's all possible because of the existence of the garbage collector, which
ensures that any objects that are no longer required are deleted. GC means that we don't have to
think too much about how we create, compose and tear down our data structures. The CLR GC will even
clean up objects that are linked in cycles.

Note that the time of garbage collection in C# is indeterminate - there is no way of knowing when an
object will be collected. This is why the `IDisposable` interface and the `using` statement exist -
to help programmers clean up resources as soon as possible. It only helps though, since it's an
"opt-in" system.

In Rust, in contrast, there is no garbage collector. First, let's talk about *values* rather than
classes. The discussion below applies not just to structs, but to fundamental types like `u8` (a
byte) and `&str` (a string slice).

* A Rust value comes into existence, and has an owner.
* There is only ever one owner.
* When the owner goes out of scope, the value is *dropped*.

'Scope' means pretty much what you would expect - you reach the end of a function:

```rs
fn function_that_takes_ownership_of_string(s: String) {
    println!("s = {}", s);
} // s is dropped here.
```

or the end of a statement block within a wider piece of code:

```rs
{   // Scope starts here
    let myFoo = Foo::new("My name");    // Create a new Foo struct, owned by myFoo
    ...
}   // myFoo goes out of scope here, and is dropped
```

For those familiar with C++, this is the RAII (Resource Acquisition Is Initialization) pattern.

You can think of dropping as equivalent to a `Dispose()` method in C# or a destructor in C++.

There are three key differences to note:

* The dropping of values is mandatory, you don't need to opt-in to it like with `IDisposable`, the
  compiler inserts the appropriate code automatically for you.
* Dropping is determinate - it happens at a known time, which is "as soon as possible" when
  the owner goes out of scope.
* Dropping is automatically recursive - if `myFoo` contains sub-structures that need dropping, they
  will be dropped automatically too.

> In case you were wondering, if there is more than one value to drop they are dropped in reverse
> order of declaration.

Of course, in the same way that not all C# classes need to implement `IDisposable`, not all Rust
values need to do something when they are dropped. The criterion is the same - if you have some data
or resource that you own and need to clean up, they you need to implement `Drop` (`u8` has no need
for a Drop method so nothing actually happens when a byte goes out of scope). In Rust, you opt-in to
a particular behaviour by *implementing a trait*, in this case the `Drop` trait.
https://doc.rust-lang.org/std/ops/trait.Drop.html A trait is a bit like an interface in C#, and the
drop trait has only one method, called `Drop`, which you must implement to get all the automatic
behavior described above.

I believe the Rust `Drop` system is superior to the C# `IDisposable` system because it happens at a
known time and is easier to use - the decision as to whether Drop is necessary is taken by the type
designer and then the compiler does all the work of inserting the necessary calls. In C#, you are
reliant on the users of your type actually calling `Dispose`. The Rust idiom is also easier to
write, `IDisposable` implementations can get tricky when inheritance is a factor - which is not a
problem in Rust because Rust does not have inheritance!

## Ownership, moves and borrowing in Rust

We briefly mentioned "ownership" above, and saw how Rust uses the lifetime of the owner to determine
when a value should be dropped. Ownership is tightly related to the concept of *move semantics* and
*borrowing*. These are Rust concepts that have no real parallel in C# and you need to understand how
they all relate in order to understand Rust.

> Ownership, move semantics and borrowing are the beating heart of Rust.

I made the statement above "There is only ever one owner." This is true, but is far from the whole
story. Firstly, the owner can change by re-assignment:

```rs
{
    // First we create a new String value owned by the variable 'a'
    let a = String::from("A String");

    // Now we change the owner of that String value by assigning it to b.
    let b = a;

    // At this point, 'a' can no longer be used because it is no longer bound to any value.
    // We say 'the String has been moved from a to b, and b is the new owner'.
    println!("This is {}", a);
}
```

This is called 'move semantics' and the fact that it occurs is the reason why Rust can guarantee to
drop values at the correct time, when their owner goes out of scope. A move can also happen when you
pass a value to a function. Consider this example:

```rs
// This function takes a string by value. When called, the argument will be moved into the parameter
// 's', which becomes the new owner of the String. The String is then dropped when it goes out of
// scope at the end of the function.
fn move_into_me(s: String) {
    println!("s = {}", s);
} // s is dropped here.

let a = String::from("A String");
move_into_me(a);
// The next line will not compile because ownership of 'a' was moved from this scope into the
// function `move_into_me`. When that function completes the String is dropped and no longer
// exists, so we cannot take the length of 'a'.
let n = a.len();
```

If all Rust had was moves, it would be a lot harder to write programs, you would basically be
reduced to writing a program as a series of function calls `f(g(h(a)))` (which is not far off from
how functional languages such as Haskell and F# work). But Rust is a multi-paradigm programming
language and needs to support the imperative programming model. This is where *borrows* come in.

A borrow is achieved by *taking a reference* to a value instead of moving it. We do this using the
`&` operator. Here is the above example, with `move_into_me` rewritten to take a reference to a
String instead of a String by value:

```rs
// This function takes a string by reference. When called, the argument will be borrowed for the
// call. Because we only borrowed the String, when the function finishes the String is not dropped,
// because 's' was never the owner.
fn take_a_reference(s: &String) {
    println!("s = {}", s);
}

let a = String::from("A String");
take_a_reference(&a);
// This now compiles fine.
let n = a.len();
```

The above example shows an *immutable reference*, which is generated by the syntax `&a`. You can
never change the thing referred to (known as 'the referent') via an immutable reference. So
attempting to add more characters to `s` inside `take_a_reference` won't compile:

```rs
fn take_a_reference(s: &String) {
    s.push("I won't compile");
}
```

To fix, this you need to use the syntax for a mutable reference, `&mut`:

```rs
// The type signature of the function now says "I take a mutable reference to a String".
fn take_a_reference(s: &mut String) {
    s.push("more");
    println!("s = {}", s);
}

let a = String::from("A String");
// We need to specify in the call that we want a mutable reference:
take_a_reference(&mut a);
let n = a.len();    // prints 12. a has changed.
```

> The English is a little unfortunate: "I take a mutable reference to a String" really means "I take
> a reference to a String and allow you to change the String via that reference", but nobody says that.

So now we have introduced owners, moves, and borrowing via immutable or mutable references. Other
programming languages let you mutate variables via any reference you can get, but Rust enforces some
rules. These are:

* You can only ever have 1 mutable reference to a value at any time
* If you have a mutable reference, you can't make an immutable reference at the same time
* If there is no mutable reference, then you can make as many immutable references as you want
* There are no dangling references, a reference always refers to valid data

These restrictions may seem strange at first, but they exist to prevent 'data races'. A data race
occurs when you have two or more pointers to the same data item, at least one of those pointers is
being used to change the underlying item, and nothing is being done to synchronize access to the
data item. The rules mean that if a value is being mutated then there is only one reference to the
value at that time, so nobody can do a dirty read. It should be obvious that the rule that says 'you
can have as many immutable references as you like' is only safe because you can't get a mutable
reference at the same time, so all those references are guaranteed to see the same data for as long
as they exist.

You've probably come across restrictions like these before: they are very similar to the semantics
of C#'s `ReaderWriterLockSlim` class, which allows multiple readers (immutable references in Rust
terminology) and one writer (a mutable reference). (Rust has no concept of upgrading a reference
though.) Database systems such as SQL Server implement similar concepts, though it gets a little
more complicated because SQL Server tries hard to optimize locking performance https://msdn.microsoft.com/en-us/library/ms186396.aspx

The restrictions mean:

* If you have an immutable reference to a value Rust **guarantees** that it won't change under you
* If you have a mutable reference, you can freely change the referent safe in the knowledge that you
  won't invalidate any other reference's view of the data (because there **are no** other
  references, mutable or immutable)

This is a pretty amazing guarantee, and is a key selling point of Rust. It means that you can write
multi-threaded programs safe in the knowledge that they are free from data races! This is known as
"fearless concurrency" in the Rust world.

> In Rust, *data* is rigorously protected by the compiler.
>
> In C#, blocks of *code* are protected with `lock` - and you'd better hope you put `lock` around
> all the places that access the synchronized resources.

However, there is a downside - you have to give up your habit of simply slapping object references
on your classes when you need access to one more piece of data. It's not that it will introduce a
bug to the Rust code - in fact, it simply won't compile! In other words, while it is fine to create
a structure like this in the C# world:

  IMAGE

In Rust, you will need to refactor to produce a DAG (directed acyclic graph) where the ownership of
each piece of data is explicitly defined in a tree structure with no cycles:

  IMAGE

Being able to write such structures from the get-go is the hardest thing to learn for a programmer
coming from C# to Rust. It affects programmers coming from other languages as well, the Rust
community even has a term for it - "fighting the borrow checker". Rest assured, it gets easier, and
the Rust compiler produces excellent error messages to help you.

## Lifetimes

Above, I mentioned this rule:

* There are no dangling references, a reference always refers to valid data

In C#, a dangling reference occurs when a reference is `null` when it shouldn't be. Trying to refer
to the referent generates the dreaded `NullReferenceException`. In low-level languages such as C and
C++ the problem also exists but behaviour is more erratic - the program may continue to run and
generate wrong answers, or it may crash. In both cases the cause is the same: we start with a value
'a' and a reference to it - '&a' - but the value 'a' went away before the reference did. This class
of errors is often called "use after free" errors.

Rust prevents such errors by guaranteeing that the value 'a' **lives for at least as long** as any
references to it. It's a powerful guarantee, and helps to eliminate an entire class of very frequent
bugs. The Rust compiler enforces this guarantee by analysing the lifetimes of your values and the
references to them. In some common cases, such as inside the body of a function, it is able to do
this automatically, without any assistance from the programmer. Rust won't allow you to return a
reference to a local variable, for example. When Rust can't figure it out, you will have to help the
compiler by providing a *lifetime annotation*.

> Lifetime annotations are sometimes needed on function signatures and whenever you have a reference
> stored inside a data structure

They look like this:

```rs
struct DefaultConfig {...};

// A reference in a struct will need a lifetime
struct Config<'a> {
    default: &'a DefaultConfig,
}

// Merge b onto a and return a.
fn merge_configs<'a>(a: &'a Config, b: &'a Config) -> &'a Config {

}

// Ditto, but using a generic type T as well as a lifetime annotation.
fn merge<'a, T>(a: &'a T, b: &'a T) -> &'a T {
}
```

The `'a`, which reads as 'tick a' or 'lifetime a', parameterizes the function in a very similar way
to the way that a C# generic function is parameterized by a type T. `'a` is very common in Rust
code, but you can use more descriptive names such as `'buffer` and you can use more than one in a
single struct or function signature.

TODO For more information see the section on lifetimes.

## Rust Enums are far more important than C# enums

In C#, an enum is just syntactic sugar for an integer. In Rust, enums are more powerful and used
much more frequently. They consist of a set of discrete values - a value of enum type can **only**
be one of the possible enumerands at any one time. (The C# concept of a FlagsEnum is represented in
Rust as a bitfield). Each one of the enumerands can have arbitrary data associated with it. The
power of enums is unleashed when combined with the `match` statement.

## Rust does not have null, it has Option<T>

Above we said Rust does not have dangling references. This means there is no concept of null, which
means `ArgumentNullExceptions` can ever happen. Instead Rust uses the generic type `Option<T>`,
which is the moral equivalent of C#'s `Nullable<T>`. However, Rust doesn't allow you to get at the
underlying value without checking that it exists via `match` pattern matching, `if let` or perhaps
`unwrap` and the `?` syntax.

Options are endemic in Rust code. They are a type of enum. Here is the actual definition from the
standard library:

```rs
pub enum Option<T> {
    None,
    Some(T)
}
```

## Rust does not have exceptions, it has Result<T,E>

There are no exceptions in Rust. If a function can fail, it can either `panic()` (bad design) or
return a type called `Result<T,E>`. Think of `Result<T,E>` as basically working the same way as
`Option<T>` but allowing a second parameter which is the error condition instead of the constant
`None`. E might be an integer error code, a string, or some other specialized value.

Results are almost as common as Options. They are also a type of enum. Here is the actual definition
from the standard library:

```rs
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

A common technique for library writers is to constrain the type of the error, E, to some specific
type and then export a new custom Result type. For example, in the standard library's IO module,
this type is defined to represent Results from IO functions (`Error` is another type defined in the
IO module):

```rs
pub type Result<T> = result::Result<T, Error>;
```

This is exactly the same technique as deriving a subclass in C# and closing one of the generic type
parameters, for example:

```cs
public class IntToSomethingDict<V> : Dictionary<int, V> { }
```
