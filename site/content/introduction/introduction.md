+++
title = "Introduction"
weight = 10
+++

## Aims of this website

Rust is often marketed as a great *system level* programming language, yet it
has plenty of high-level features which make it similar to languages such as C#
or Java. At the same time, it is different enough to make learning it
non-trivial.

My aim in writing this website is to make a comprehensive reference which
experienced C# programmers can use to translate their knowledge into equivalent
Rust concepts with as little hunting around and Googling as possible. The search
box at the top of the screen will do a free-text search against the whole site,
which means that you can type in virtually any C# concept (System.Byte,
File.ReadAllLines, NuGet) and quickly find material on each one.

The mapping between C# and Rust is not complete, and in fact never can be. C#
has some concepts such as Reflection which simply don't exist in the Rust world.
The reverse is also true: C# has nothing like the Rust concept of ownership and
borrowing - in fact, no other language has these features, they are unique to
Rust, though as we shall see the principles behind them have some surprising
analogues.

At the time of writing this (Feb 2018) the Rust ecosystem is not as mature as
the C# one, which means that some libraries such as database access do not have
production quality equivalents, and I will have virtually nothing to say about
GUI development. On the other hand, Rust has some rapidly improving frameworks
for HTTP server and client development which means writing a RESTful server is
as easy in Rust as it is in C# WebApi.

The site can be used much like a "super cheat-sheet", but the sections are
arranged in a logical order, and you definitely should read all of the
introduction and these sections which constitute things you just have to know:

* Ownership and borrowing
* Option and Result
* String and &str
* Tuples and Structs
* Sequential collections: Vec, arrays and slices
* Associative collections: HashMap and Set
* Iterators
* Closures
* Generics
* Traits and trait objects
* Utility traits
* How to read Rust function signatures

## Thoughts on learning a new programming language

Learning a new programming language can be a frustrating experience. Problems
that you consider to be trivial because you can solve them almost without
thinking in your first language may take 10 times longer or even defeat you
altogether in the new language. Knowledge and techniques you have grown to rely
on simply may not work in the new language. The development environment doesn't
work the same way, the error messages make no sense, and it's harder to know
where to go to get help or even what to Google for.

Everybody will deal with this in their own way, but here are a few suggestions:

* As a first pass, just go read Blandy and Orendorrf from cover to cover, or the
  online Rust book. Don't worry about coding, just read the whole thing and
  allow the major concepts to sink in.
* Learn the major 'terms of art' in the Rust world - it will make Googling for
  specific things easier.
* Don't fret the details: you really don't have to worry about numeric overflows
  and operator overloading to get started. You can always come back to them if
  you ever need to. It's like operator precedence - sure it's important, and
  every reference book ever written for any programming language seems to have a
  massive table of available operators and their precedence round about Chapter
  2, but most modern languages work roughly the same way and franly, you can
  wing it to start with.
* The only way to achieve proficiency is to practice - write code, lots of it.
* Read other people's code, including the standard library source code.
* Small victories matter: don't start off by trying to write a compiler, make
  small console apps.
* Try porting code that you already know well. If your first Rust programs are
  attempts to solve new problems you will be trying to do two things at once:
  solve the problem, and figure out how to express the solution in Rust. If you
  port existing code, the problem is already solved and you only need to figure
  out how to express it. Once you've done that, figure out the *best* way to
  express it.
* You'll probably spend quite a lot of time wrestling with the compiler at first
  - it is easier to make small refactorings to a working program than it is to
  make lots of changes at once.
* There is a tendency in the Rust community to emphasize efficient,
  non-allocating code. Don't worry too much about this at first, if adding a
  call to `clone` or `to_string` will get your program working, do it.
  Eventually you'll internalize the techniques for writing the most efficient
  code the first time.
* If something isn't working for you, stop and do something else. Come back to
  it later.
* Learning will be a gestalten experience - eventually the little bits of
  hard-won knowledge will coalesce into a coherent whole and you will be 'over
  the hump' and things will get a lot easier.
* The Rust compiler generates extremely good error messages: learn to understand
  them.
* Try contributing fixes to Rust open-source projects. You will get the benefit
  of more experienced programmers reviewing your code.
  https://www.rustaceans.org/findwork/
* Blog about your Rust experiences. Explaining to others is a great way of
  checking your own understanding - in fact, it's one of the main reasons that I
  created this site!
* Read the questions on online forums such as The Rust Language Forum and Stack
  Overflow.
* Keep a positive mental attitude and accept that it is going to take a
  considerable amount of time and hard work.

## Main References

I expect you to use this site in conjunction with other reference sources. I
strongly recommend you buy the 'Programming Rust' book by Blandy and Orendorff.
It does a much better job of explaining the language than the online 'Rust Book'
website.

* Programming Rust by Blandy and Orendorff
* The Rust Book
* The Cargo reference manual
* The Rust standard library: Document and Source Code
* C# in a Nutshell Albahari
* CLR via C# by Jeffrey Richter

## Secondary References

When you install Rust I recommend you install the `rust-src` component, which
gives you the source code to the `std` library. It is an invaluable resource for
seeing how Rust professionals write code, and I think you will be surprised by
how simple and understandable much of it is. It's an invaluable learning
resource.

* Rust questions on Stack Overflow
* https://codereview.stackexchange.com/questions/tagged/rust
* The Rust Language Forum
* Reddit /r/rust
* Find something Rusty to work on: https://www.rustaceans.org/findwork/
